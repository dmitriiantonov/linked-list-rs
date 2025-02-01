use std::marker::PhantomData;
use std::ptr::NonNull;

/// A type alias for a nullable pointer to a `Node<T>`.
///
/// This type alias is used to represent a nullable pointer to a node,
/// making it easier to work with linked list pointers.
type Link<T> = Option<NonNull<Node<T>>>;

/// Represents a node in the doubly linked list.
///
/// Each node contains a value (`element`), a pointer to the next node (`next`),
/// and a pointer to the previous node (`prev`), enabling bidirectional traversal of the list.
///
/// # Fields
/// - `element`: The value stored in the node.
/// - `next`: A pointer to the next node in the list, or `None` if there is no next node.
/// - `prev`: A pointer to the previous node in the list, or `None` if there is no previous node.
#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

/// A doubly linked list.
///
/// This list supports insertion and deletion at both ends with constant time complexity.
/// It allows efficient traversal in both directions (from head to tail and vice versa).
///
/// # Fields
/// - `head`: A pointer to the first node in the list, or `None` if the list is empty.
/// - `tail`: A pointer to the last node in the list, or `None` if the list is empty.
/// - `len`: The number of elements in the list.
/// - `_marker`: A marker to indicate the ownership of the elements (`T`) without actually storing them.
///
/// # Example
/// ```
/// use linked_list::LinkedList;
/// 
/// let mut list = LinkedList::new();
/// list.push_front(1);
/// list.push_back(2);
/// ```
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    _marker: PhantomData<T>,
}

/// An iterator that consumes the linked list.
///
/// The `IntoIter` struct consumes the linked list as it iterates, meaning the list is moved
/// and cannot be accessed after the iteration starts.
///
/// # Fields
/// - `list`: The `LinkedList` that is being consumed by the iterator.
pub struct IntoIter<T> {
    list: LinkedList<T>,
}

/// An immutable iterator over the linked list.
///
/// The `Iter` struct allows you to traverse a linked list in a read-only manner, without modifying the list.
/// The iterator keeps track of the remaining elements in the iteration and provides access to them.
///
/// # Fields
/// - `head`: A pointer to the first node in the iteration, or `None` if the iteration has finished.
/// - `tail`: A pointer to the last node in the iteration, or `None` if the iteration has finished.
/// - `len`: The number of elements remaining in the iteration.
/// - `_marker`: A marker to indicate the lifetime of the borrowed nodes.
///
/// # Example
/// ```
/// use linked_list::LinkedList;
/// 
/// let list = LinkedList::new();
/// let mut iter = list.iter();
/// ```
pub struct Iter<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
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

/// A cursor for immutable access to the nodes of a `LinkedList`.
///
/// The `Cursor` struct provides an iterator-like interface for traversing a `LinkedList` without modifying it.
/// You can use it to access the elements of the list and move through the list sequentially.
///
/// # Fields
/// - `index`: The current position of the cursor within the list, starting from 0.
/// - `current`: A non-null pointer to the current node that the cursor is pointing to, or `None` if the cursor is at the end of the list.
/// - `list`: A reference to the `LinkedList` being iterated over.
///
/// # Example
/// ```
/// use linked_list::LinkedList;
/// 
/// let mut list = LinkedList::new();
/// let mut cursor = list.cursor();
/// ```
pub struct Cursor<'a, T: 'a> {
    index: usize,
    current: Option<NonNull<Node<T>>>,
    list: &'a LinkedList<T>,
}

/// A cursor for mutable access to the nodes of a `LinkedList`.
///
/// The `CursorMut` struct allows both reading from and modifying the nodes of a `LinkedList`.
/// It provides an iterator-like interface with the ability to change the state of the list as you traverse it.
///
/// # Fields
/// - `index`: The current position of the cursor within the list, starting from 0.
/// - `current`: A non-null pointer to the current node that the cursor is pointing to, or `None` if the cursor is at the end of the list.
/// - `list`: A mutable reference to the `LinkedList` being iterated over, allowing modification of its nodes.
///
/// # Example
/// ```
/// use linked_list::LinkedList;
/// 
/// let mut list = LinkedList::new();
/// let mut cursor = list.cursor_mut();
/// ```
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

    #[inline]
    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut::new(self)
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

/// Moves the cursor to the next element in the list.
fn move_next<T>(current: &mut Link<T>, index: &mut  usize, list: &LinkedList<T>) {
    match current {
        None => {
            *current = list.head;
            *index = 0;
        }
        Some(node) => unsafe {
            *current = node.as_ref().next;
            *index += 1;
        },
    }
}

/// Moves the cursor to the previous element in the list.
fn move_prev<T>(current: &mut Link<T>, index: &mut  usize, list: &LinkedList<T>) {
    match current {
        None => {
           *current = list.tail;
           *index = list.len().saturating_sub(1)
        }
        Some(node) => unsafe {
            *current = node.as_ref().prev;
            *index = index.saturating_sub(1)
        },
    }
}

impl<'a, T: 'a> Cursor<'a, T> {
    /// Creates a new `Cursor` positioned at the start of the list.
    #[inline]
    pub fn new(list: &'a LinkedList<T>) -> Self {
        Self {
            index: 0,
            current: None,
            list,
        }
    }

    /// Returns the current index if the cursor is pointing to an element.
    #[inline]
    pub fn index(&self) -> Option<usize> {
        self.current.map(|_| self.index)
    }

    /// Returns a reference to the current element, if any.
    #[inline]
    pub fn current(&mut self) -> Option<&'a T> {
        self.current
            .map(|node| unsafe { &(*node.as_ptr()).element })
    }

    /// Moves the cursor to the next element in the list.
    pub fn move_next(&mut self) {
        move_next(&mut self.current, &mut self.index, self.list);
    }

    /// Moves the cursor to the previous element in the list.
    pub fn move_prev(&mut self) {
        move_prev(&mut self.current, &mut self.index, self.list);
    }

    /// Peeks at the next element without moving the cursor.
    pub fn peek_next(&self) -> Option<&'a T> {
        unsafe {
            let next = match self.current {
                None => self.list.head,
                Some(node) => node.as_ref().next,
            };
            next.map(|node| &(*node.as_ptr()).element)
        }
    }

    /// Peeks at the previous element without moving the cursor.
    pub fn peek_prev(&self) -> Option<&'a T> {
        unsafe {
            let prev = match self.current {
                None => self.list.tail,
                Some(current) => current.as_ref().prev,
            };
            prev.map(|node| &(*node.as_ptr()).element)
        }
    }
}

impl<'a, T: 'a> CursorMut<'a, T> {
    /// Creates a new `CursorMut` positioned at the start of the list.
    #[inline]
    pub fn new(list: &'a mut LinkedList<T>) -> Self {
        Self {
            index: 0,
            current: None,
            list,
        }
    }

    /// Returns the current index if the cursor is pointing to an element.
    #[inline]
    pub fn index(&mut self) -> Option<usize> {
        let _ = self.current?;
        Some(self.index)
    }

    /// Returns a mutable reference to the current element, if any.
    #[inline]
    pub fn current(&mut self) -> Option<&'a mut T> {
        self.current
            .map(|node| unsafe { &mut (*node.as_ptr()).element })
    }

    /// Moves the cursor to the next element in the list.
    pub fn move_next(&mut self) {
        move_next(&mut self.current, &mut self.index, self.list);
    }

    /// Moves the cursor to the previous element in the list.
    pub fn move_prev(&mut self) {
        move_prev(&mut self.current, &mut self.index, self.list);
    }

    /// Peeks at the next element without moving the cursor.
    pub fn peek_next(&self) -> Option<&'a mut T> {
        unsafe {
            let next = match self.current {
                None => self.list.head,
                Some(current) => current.as_ref().next,
            };

            next.map(|node| &mut (*node.as_ptr()).element)
        }
    }

    /// Peeks at the previous element without moving the cursor.
    pub fn peek_prev(&self) -> Option<&'a mut T> {
        unsafe {
            let prev = match self.current {
                None => self.list.tail,
                Some(current) => current.as_ref().prev,
            };

            prev.map(|node| &mut (*node.as_ptr()).element)
        }
    }

    /// Deletes the current element and moves the cursor to the next element.
    pub fn delete(&mut self) -> Option<T> {
        unsafe {
            self.current.map(|node| {
                let current = Box::from_raw(node.as_ptr());
                let prev = current.prev;
                let next = current.next;

                match prev {
                    None => {
                        self.list.head = next;
                    }
                    Some(prev) => {
                        (*prev.as_ptr()).next = next;
                    }
                }

                if let Some(next) = next {
                    (*next.as_ptr()).prev = prev
                }

                self.current = next;

                if next.is_none() {
                    self.list.tail = prev;
                }

                self.current = next;

                current.element
            })
        }
    }

    /// Inserts an element before the current position.
    pub fn insert_before(&mut self, element: T) {
        match self.current {
            None => {
                self.list.push_front(element);
            }
            Some(current) => unsafe {
                let prev = current.as_ref().prev;

                match prev {
                    None => {
                        self.list.push_front(element);
                    }
                    Some(prev) => {
                        let node = NonNull::from(Box::leak(Box::new(Node::new(element))));
                        (*prev.as_ptr()).next = Some(node);
                        (*node.as_ptr()).prev = Some(prev);
                        (*node.as_ptr()).next = Some(current);
                        (*current.as_ptr()).prev = Some(node);
                        self.list.len += 1;
                    }
                }

                self.index += 1;
            },
        }
    }

    /// Inserts an element after the current position.
    pub fn insert_after(&mut self, element: T) {
        match self.current {
            None => {
                self.list.push_back(element);
            }
            Some(current) => unsafe {
                let next = current.as_ref().next;

                match next {
                    None => {
                        self.list.push_back(element);
                    }
                    Some(next) => {
                        let node = NonNull::from(Box::leak(Box::new(Node::new(element))));
                        (*current.as_ptr()).next = Some(node);
                        (*node.as_ptr()).prev = Some(current);
                        (*node.as_ptr()).next = Some(next);
                        (*next.as_ptr()).prev = Some(node);
                        self.list.len += 1;
                    }
                }
            },
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
    fn test_cursor_move_next() {
        let mut list = LinkedList::from([1, 2, 3]);
        let mut cursor = list.cursor();

        assert_eq!(cursor.current, None);
        assert_eq!(cursor.index(), None);

        cursor.move_next();

        assert_eq!(cursor.current(), Some(&1));
        assert_eq!(cursor.index(), Some(0));

        cursor.move_next();

        assert_eq!(cursor.current(), Some(&2));
        assert_eq!(cursor.index(), Some(1));

        cursor.move_next();

        assert_eq!(cursor.current(), Some(&3));
        assert_eq!(cursor.index(), Some(2));

        cursor.move_next();

        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.index(), None);
    }

    #[test]
    fn test_cursor_move_back() {
        let mut list = LinkedList::from([1, 2, 3]);
        let mut cursor = list.cursor();

        assert_eq!(cursor.current, None);
        assert_eq!(cursor.index(), None);

        cursor.move_prev();

        assert_eq!(cursor.current(), Some(&3));
        assert_eq!(cursor.index(), Some(2));

        cursor.move_prev();

        assert_eq!(cursor.current(), Some(&2));
        assert_eq!(cursor.index(), Some(1));

        cursor.move_prev();

        assert_eq!(cursor.current(), Some(&1));
        assert_eq!(cursor.index(), Some(0));

        cursor.move_prev();

        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.index(), None);
    }

    #[test]
    fn test_cursor_peek_next() {
        let mut list = LinkedList::from([1, 2, 3]);
        let mut cursor = list.cursor();

        assert_eq!(cursor.peek_next(), Some(&1));

        cursor.move_next();

        assert_eq!(cursor.peek_next(), Some(&2));

        cursor.move_next();

        assert_eq!(cursor.peek_next(), Some(&3));

        cursor.move_next();

        assert_eq!(cursor.peek_next(), None);
    }

    #[test]
    fn test_cursor_peek_prev() {
        let mut list = LinkedList::from([1, 2, 3]);
        let mut cursor = list.cursor();

        assert_eq!(cursor.peek_prev(), Some(&3));

        cursor.move_prev();

        assert_eq!(cursor.peek_prev(), Some(&2));

        cursor.move_prev();

        assert_eq!(cursor.peek_prev(), Some(&1));

        cursor.move_prev();

        assert_eq!(cursor.peek_prev(), None);
    }

    #[test]
    fn test_cursor_mut_move_next() {
        let mut list = LinkedList::from([1, 2, 3]);
        let mut cursor = list.cursor_mut();

        assert_eq!(cursor.current, None);
        assert_eq!(cursor.index(), None);

        cursor.move_next();

        assert_eq!(cursor.current(), Some(&mut 1));
        assert_eq!(cursor.index(), Some(0));

        cursor.move_next();

        assert_eq!(cursor.current(), Some(&mut 2));
        assert_eq!(cursor.index(), Some(1));

        cursor.move_next();

        assert_eq!(cursor.current(), Some(&mut 3));
        assert_eq!(cursor.index(), Some(2));

        cursor.move_next();

        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.index(), None);
    }

    #[test]
    fn test_cursor_mut_move_back() {
        let mut list = LinkedList::from([1, 2, 3]);
        let mut cursor = list.cursor_mut();

        assert_eq!(cursor.current, None);
        assert_eq!(cursor.index(), None);

        cursor.move_prev();

        assert_eq!(cursor.current(), Some(&mut 3));
        assert_eq!(cursor.index(), Some(2));

        cursor.move_prev();

        assert_eq!(cursor.current(), Some(&mut 2));
        assert_eq!(cursor.index(), Some(1));

        cursor.move_prev();

        assert_eq!(cursor.current(), Some(&mut 1));
        assert_eq!(cursor.index(), Some(0));

        cursor.move_prev();

        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.index(), None);
    }

    #[test]
    fn test_cursor_mut_peek_next() {
        let mut list = LinkedList::from([1, 2, 3]);
        let mut cursor = list.cursor_mut();

        assert_eq!(cursor.peek_next(), Some(&mut 1));

        cursor.move_next();

        assert_eq!(cursor.peek_next(), Some(&mut 2));

        cursor.move_next();

        assert_eq!(cursor.peek_next(), Some(&mut 3));

        cursor.move_next();

        assert_eq!(cursor.peek_next(), None);
    }

    #[test]
    fn test_cursor_mut_peek_prev() {
        let mut list = LinkedList::from([1, 2, 3]);
        let mut cursor = list.cursor_mut();

        assert_eq!(cursor.peek_prev(), Some(&mut 3));

        cursor.move_prev();

        assert_eq!(cursor.peek_prev(), Some(&mut 2));

        cursor.move_prev();

        assert_eq!(cursor.peek_prev(), Some(&mut 1));

        cursor.move_prev();

        assert_eq!(cursor.peek_prev(), None);
    }

    #[test]
    fn test_cursor_mut_delete_head() {
        let mut list = LinkedList::from([1, 2, 3]);
        let mut cursor = list.cursor_mut();

        cursor.move_next();
        let deleted = cursor.delete();

        assert_eq!(deleted, Some(1));

        assert_eq!(cursor.current(), Some(&mut 2));
        assert_eq!(cursor.index(), Some(0));

        let values = list.into_iter().collect::<Vec<_>>();

        assert_eq!(values, vec![2, 3]);
    }

    #[test]
    fn test_cursor_mut_delete_mid() {
        let mut list = LinkedList::from([1, 2, 3]);
        let mut cursor = list.cursor_mut();

        cursor.move_next();
        cursor.move_next();

        let deleted = cursor.delete();
        assert_eq!(deleted, Some(2));

        assert_eq!(cursor.current(), Some(&mut 3));
        assert_eq!(cursor.index(), Some(1));

        let values = list.into_iter().collect::<Vec<_>>();

        assert_eq!(values, vec![1, 3]);
    }

    #[test]
    fn test_cursor_mut_delete_tail() {
        let mut list = LinkedList::from([1, 2, 3]);
        let mut cursor = list.cursor_mut();

        cursor.move_next();
        cursor.move_next();
        cursor.move_next();

        let deleted = cursor.delete();
        assert_eq!(deleted, Some(3));

        assert_eq!(cursor.current(), None);

        let values = list.into_iter().collect::<Vec<_>>();

        assert_eq!(values, vec![1, 2]);
    }

    #[test]
    fn test_cursor_mut_insert_before_when_current_empty() {
        let mut list = LinkedList::from([2, 3]);
        let mut cursor = list.cursor_mut();

        cursor.insert_before(1);

        let values = list.into_iter().collect::<Vec<_>>();

        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_cursor_mut_insert_after_when_prev_empty() {
        let mut list = LinkedList::from([2, 3]);
        let mut cursor = list.cursor_mut();

        cursor.move_next();

        cursor.insert_before(1);

        assert_eq!(cursor.current(), Some(&mut 2));
        assert_eq!(cursor.index(), Some(1));

        let values = list.into_iter().collect::<Vec<_>>();

        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_cursor_mut_insert_before() {
        let mut list = LinkedList::from([1, 3]);
        let mut cursor = list.cursor_mut();

        cursor.move_next();
        cursor.move_next();

        cursor.insert_before(2);

        assert_eq!(cursor.current(), Some(&mut 3));
        assert_eq!(cursor.index(), Some(2));

        let values = list.into_iter().collect::<Vec<_>>();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_cursor_mut_insert_after_when_current_empty() {
        let mut list = LinkedList::from([1, 2]);
        let mut cursor = list.cursor_mut();

        cursor.insert_after(3);

        let values = list.into_iter().collect::<Vec<i32>>();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_cursor_mut_insert_after_when_next_empty() {
        let mut list = LinkedList::from([1, 2]);
        let mut cursor = list.cursor_mut();

        cursor.move_next();
        cursor.move_next();

        cursor.insert_after(3);

        let values = list.into_iter().collect::<Vec<i32>>();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_cursor_mut_insert_after() {
        let mut list = LinkedList::from([1, 3]);
        let mut cursor = list.cursor_mut();

        cursor.move_next();

        cursor.insert_after(2);

        let values = list.into_iter().collect::<Vec<i32>>();
        assert_eq!(values, vec![1, 2, 3]);
    }
}
