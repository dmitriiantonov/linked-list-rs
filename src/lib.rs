use std::marker::PhantomData;
use std::ptr::NonNull;

/// A type alias for a nullable pointer to a `Node<T>`.
type Link<T> = Option<NonNull<Node<T>>>;

/// Represents a node in the doubly linked list.
#[derive(Debug)]
struct Node<T> {
    /// The value stored in the node.
    element: T,
    /// Pointer to the next node in the list.
    next: Option<NonNull<Node<T>>>,
    /// Pointer to the previous node in the list.
    prev: Option<NonNull<Node<T>>>,
}

/// A doubly linked list.
///
/// This list allows insertion and deletion from both ends in constant time.
pub struct LinkedList<T> {
    /// Pointer to the first node in the list.
    head: Link<T>,
    /// Pointer to the last node in the list.
    tail: Link<T>,
    /// Number of elements in the list.
    len: usize,
    /// Marker to indicate ownership of `T` without actually storing it.
    _marker: PhantomData<T>,
}

/// An iterator that consumes the linked list.
pub struct IntoIter<T> {
    /// The linked list being iterated over.
    list: LinkedList<T>,
}

/// An immutable iterator over the linked list.
pub struct Iter<'a, T: 'a> {
    /// Pointer to the first node in the iteration.
    head: Option<NonNull<Node<T>>>,
    /// Pointer to the last node in the iteration.
    tail: Option<NonNull<Node<T>>>,
    /// Number of elements remaining in the iterator.
    len: usize,
    /// Marker to indicate borrowing of `Node<T>`.
    _marker: PhantomData<&'a Node<T>>,
}

/// A mutable iterator over the linked list.
pub struct IterMut<'a, T: 'a> {
    /// Pointer to the first node in the iteration.
    head: Option<NonNull<Node<T>>>,
    /// Pointer to the last node in the iteration.
    tail: Option<NonNull<Node<T>>>,
    /// Number of elements remaining in the iterator.
    len: usize,
    /// Marker to indicate mutable borrowing of `Node<T>`.
    _marker: PhantomData<&'a mut Node<T>>,
}

pub struct Cursor<'a, T: 'a> {
    index: usize,
    current: Option<NonNull<Node<T>>>,
    list: &'a LinkedList<T>,
}

pub struct CursorMut<'a, T: 'a> {
    index: usize,
    current: Option<NonNull<Node<T>>>,
    list: &'a mut LinkedList<T>,
}

impl<T> Node<T> {
    /// Creates a new node with the given element.
    ///
    /// # Arguments
    /// * `element` - The value to be stored in the node.
    fn new(element: T) -> Self {
        Self {
            element,
            next: None,
            prev: None,
        }
    }
}

/// A doubly linked list implementation.
///
/// This linked list allows insertion and deletion from both ends in constant time.
impl<T> LinkedList<T> {
    /// Creates a new empty linked list.
    #[inline]
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            _marker: Default::default(),
        }
    }

    /// Returns the number of elements in the linked list.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the linked list contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Adds an element to the front of the list.
    ///
    /// # Arguments
    /// * `value` - The value to insert at the front.
    pub fn push_front(&mut self, value: T) {
        let new_head = NonNull::from(Box::leak(Box::new(Node::new(value))));

        match self.head.take() {
            None => {
                self.tail = Some(new_head);
            }
            Some(old_head) => unsafe {
                (*old_head.as_ptr()).prev = Some(new_head);
                (*new_head.as_ptr()).next = Some(old_head);
            },
        }

        self.head = Some(new_head);
        self.len += 1;
    }

    /// Adds an element to the back of the list.
    ///
    /// # Arguments
    /// * `value` - The value to insert at the back.
    pub fn push_back(&mut self, value: T) {
        let new_tail = NonNull::from(Box::leak(Box::new(Node::new(value))));

        match self.tail.take() {
            None => {
                self.head = Some(new_tail);
            }
            Some(old_tail) => unsafe {
                (*old_tail.as_ptr()).next = Some(new_tail);
                (*new_tail.as_ptr()).prev = Some(old_tail);
            },
        }

        self.tail = Some(new_tail);
        self.len += 1;
    }

    /// Removes and returns the element from the front of the list.
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| unsafe {
            let mut old_head = Box::from_raw(old_head.as_ptr());

            self.head = match old_head.next.take() {
                None => {
                    self.tail = None;
                    None
                }
                Some(new_tail) => {
                    (*new_tail.as_ptr()).prev = None;
                    Some(new_tail)
                }
            };

            self.len -= 1;
            old_head.element
        })
    }

    /// Removes and returns the element from the back of the list.
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| unsafe {
            let mut old_tail = Box::from_raw(old_tail.as_ptr());

            self.tail = match old_tail.prev.take() {
                None => {
                    self.head = None;
                    None
                }
                Some(new_tail) => {
                    (*new_tail.as_ptr()).next = None;
                    Some(new_tail)
                }
            };

            self.len -= 1;
            old_tail.element
        })
    }

    /// Returns a reference to the first element of the list, if any.
    #[inline]
    pub fn front(&self) -> Option<&T> {
        self.head.map(|node| unsafe { &(*node.as_ptr()).element })
    }

    /// Returns a mutable reference to the first element of the list, if any.
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head
            .map(|node| unsafe { &mut (*node.as_ptr()).element })
    }

    /// Returns a reference to the last element of the list, if any.
    #[inline]
    pub fn back(&self) -> Option<&T> {
        self.tail.map(|node| unsafe { &(*node.as_ptr()).element })
    }

    /// Returns a mutable reference to the last element of the list, if any.
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.tail
            .map(|node| unsafe { &mut (*node.as_ptr()).element })
    }

    /// Returns an iterator over the elements of the list.
    #[inline]
    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            _marker: Default::default(),
        }
    }

    /// Returns a mutable iterator over the elements of the list.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len,
            _marker: Default::default(),
        }
    }

    #[inline]
    pub fn cursor(&mut self) -> Cursor<T> {
        Cursor::new(self)
    }
}

impl<T> Default for LinkedList<T> {
    #[inline]
    fn default() -> Self {
        LinkedList::new()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<T> Drop for LinkedList<T> {
    #[inline]
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}

impl<T, const N: usize> From<[T; N]> for LinkedList<T> {
    fn from(array: [T; N]) -> Self {
        let mut list = Self::default();
        for elem in array {
            list.push_back(elem);
        }
        list
    }
}

impl<E> FromIterator<E> for LinkedList<E> {
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        let mut list = Self::default();
        for elem in iter {
            list.push_back(elem);
        }
        list
    }
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                let node = &*node.as_ptr();
                self.head = node.next;
                self.len -= 1;
                &node.element
            })
        }
    }
}

impl<'a, T: 'a> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| unsafe {
                let node = &*node.as_ptr();
                self.len -= 1;
                self.tail = node.prev;
                &node.element
            })
        }
    }
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                let node = &mut *node.as_ptr();
                self.len -= 1;
                self.head = node.next;
                &mut node.element
            })
        }
    }
}

impl<'a, T: 'a> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| unsafe {
                let node = &mut *node.as_ptr();
                self.len -= 1;
                self.tail = node.prev;
                &mut node.element
            })
        }
    }
}

impl<'a, T: 'a> Cursor<'a, T> {
    #[inline]
    pub fn new(list: &'a LinkedList<T>) -> Self {
        Self {
            index: 0,
            current: None,
            list,
        }
    }

    #[inline]
    pub fn index(&self) -> Option<usize> {
        self.current.map(|_| self.index)
    }

    #[inline]
    pub fn current(&self) -> Option<&T> {
        self.current
            .map(|node| unsafe { &(*node.as_ptr()).element })
    }

    pub fn move_next(&mut self) {
        match self.current {
            None => {
                self.current = self.list.head;
                self.index = 0;
            }
            Some(current) => unsafe {
                self.current = current.as_ref().next;
                self.index += 1;
            },
        }
    }

    pub fn move_prev(&mut self) {
        match self.current {
            None => {
                self.current = self.list.tail;
                self.index = self.list.len().saturating_sub(1)
            }
            Some(current) => unsafe {
                self.current = current.as_ref().prev;
                self.index = self.index.saturating_sub(1)
            },
        }
    }


    pub fn peek_next(&self) -> Option<&T> {
        unsafe {
            let next = match self.current {
                None => self.list.head,
                Some(node) => node.as_ref().next,
            };
            next.map(|node| &(*node.as_ptr()).element)
        }
    }


    pub fn peek_prev(&self) -> Option<&T> {
        unsafe {
            let prev = match self.current {
                None => self.list.tail,
                Some(current) => current.as_ref().prev,
            };
            prev.map(|node| &(*node.as_ptr()).element)
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front() {
        let mut list = LinkedList::new();
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_push_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_front() {
        let mut list = LinkedList::new();

        list.push_front(3);
        assert_eq!(list.front(), Some(&3));

        list.push_front(2);
        assert_eq!(list.front(), Some(&2));

        list.push_front(1);
        assert_eq!(list.front(), Some(&1));
    }

    #[test]
    fn test_back() {
        let mut list = LinkedList::new();

        list.push_back(1);
        assert_eq!(list.back(), Some(&1));

        list.push_back(2);
        assert_eq!(list.back(), Some(&2));

        list.push_back(3);
        assert_eq!(list.back(), Some(&3));
    }

    #[test]
    fn test_front_mut() {
        let mut list = LinkedList::new();

        list.push_front(3);
        assert_eq!(list.front_mut(), Some(&mut 3));

        if let Some(element) = list.front_mut() {
            *element = 2;
        }

        assert_eq!(list.pop_front(), Some(2));
    }

    #[test]
    fn test_back_mut() {
        let mut list = LinkedList::new();

        list.push_back(1);
        assert_eq!(list.back_mut(), Some(&mut 1));

        if let Some(elem) = list.back_mut() {
            *elem = 2;
        }

        assert_eq!(list.pop_back(), Some(2));
    }

    #[test]
    fn test_iter() {
        let list = LinkedList::from([1, 2, 3, 4, 5]);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next_back(), Some(&5));
        assert_eq!(iter.next_back(), Some(&4));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut list = LinkedList::from([1, 2, 3, 4, 5]);

        let mut iter = list.iter_mut();

        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next_back(), Some(&mut 5));
        assert_eq!(iter.next_back(), Some(&mut 4));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_len() {
        let mut list = LinkedList::from([1, 2, 3]);
        assert_eq!(list.len(), 3);
        list.pop_back();
        assert_eq!(list.len(), 2);
        list.pop_front();
        assert_eq!(list.len(), 1);
        list.pop_front();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_is_empty() {
        let mut list = LinkedList::new();
        assert!(list.is_empty());
        list.push_back(1);
        assert!(!list.is_empty());
        list.pop_back();
        assert!(list.is_empty());
    }

    #[test]
    fn test_from_iter() {
        let elements = [1, 2, 3];
        let mut list = LinkedList::from_iter(elements.iter());

        assert_eq!(list.pop_front(), Some(&1));
        assert_eq!(list.pop_front(), Some(&2));
        assert_eq!(list.pop_front(), Some(&3));
    }

    #[test]
    fn test_into_iter() {
        let list = LinkedList::from([1, 2, 3, 4, 5]);
        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), Some(5));
        assert_eq!(iter.next_back(), Some(4));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_cursor() {
        let mut list = LinkedList::from([1, 2, 3]);
        let mut cursor = list.cursor();

        assert_eq!(cursor.current(), None);

        assert_eq!(cursor.peek_next(), Some(&1));

        cursor.move_next();

        assert_eq!(cursor.current(), Some(&1));
        assert_eq!(cursor.index(), Some(0));

        assert_eq!(cursor.peek_next(), Some(&2));

        cursor.move_next();

        assert_eq!(cursor.current(), Some(&2));
        assert_eq!(cursor.index(), Some(1));

        assert_eq!(cursor.peek_next(), Some(&3));

        cursor.move_next();

        assert_eq!(cursor.current(), Some(&3));
        assert_eq!(cursor.index(), Some(2));
        
        assert_eq!(cursor.peek_next(), None);

        cursor.move_next();
        assert_eq!(cursor.current(), None);
        
        assert_eq!(cursor.peek_prev(), Some(&3));

        cursor.move_prev();

        assert_eq!(cursor.current(), Some(&3));
        assert_eq!(cursor.index(), Some(2));

        assert_eq!(cursor.peek_prev(), Some(&2));

        cursor.move_prev();

        assert_eq!(cursor.current(), Some(&2));
        assert_eq!(cursor.index(), Some(1));

        assert_eq!(cursor.peek_prev(), Some(&1));

        cursor.move_prev();

        assert_eq!(cursor.current(), Some(&1));
        assert_eq!(cursor.index(), Some(0));
    }
}
