#include <cassert>
#include <cstdlib>
#include <iostream>
#include <cstring>

template <typename T> struct Node {
  T data;
  Node *next;

  explicit Node(const T &value) noexcept : data(value), next(nullptr) {}

  bool operator==(const Node &rhs) const noexcept {
    if (!next && !rhs.next) {
      return data == rhs.data;
    } else if (!next || !rhs.next) {
      return false;
    }
    return data == rhs.data && *next == *rhs.next;
  }

  bool operator!=(const Node &rhs) noexcept { return !(this->operator==(rhs)); }

  static Node *create(const T &value) noexcept {
    Node *newNode = static_cast<Node *>(std::malloc(sizeof(Node)));
    std::memset(newNode, 0, sizeof(Node));
    newNode->data = value;
    newNode->next = nullptr;
    return newNode;
  }
};

template <typename T> class LinkedList {
public:
  Node<T> *head = nullptr;

  LinkedList() noexcept : head(nullptr) {}

  ~LinkedList() noexcept {
    Node<T> *curr = head;
    while (curr) {
      Node<T> *next = curr->next;
      curr->~Node();
      free(curr);
      curr = next;
    }
  }

  bool operator==(const LinkedList &rhs) const noexcept {
    if (!head && !rhs.head) {
      return true;
    } else if (!head || !rhs.head) {
      return false;
    }
    return (*head == *rhs.head);
  }

  bool operator!=(const LinkedList &rhs) noexcept { return !(*this == rhs); }

  void push(const T &value) noexcept {
    Node<T> *newNode = Node<T>::create(value);

    // If the list is empty, set the new node as the head
    if (head == nullptr) {
      head = newNode;
    } else {
      // Otherwise, update the links
      newNode->next = head;
      head = newNode;
    }
  }

  void push_back(const T &value) noexcept {
    Node<T> *newNode = Node<T>::create(value);

    // If the list is empty, set the new node as the head
    if (head == nullptr) {
      head = newNode;
    } else {
      // Otherwise, find the last node and update the links
      Node<T> *curr = head;
      while (curr->next) {
        curr = curr->next;
      }
      curr->next = newNode;
    }
  }

  T pop() noexcept {
    assert(head && "Insufficient stack items.");
    T value = head->data;

    // Update the head
    Node<T> *next = head->next;
    std::free(head);
    head = next;

    return value;
  }

  void clear() noexcept {
    while (head) {
      Node<T> *next = head->next;
      head->~Node();
      std::free(head);
      head = next;
    }
  }

  T front() noexcept {
    assert(head && "Insufficient stack items.");
    return head->data;
  }

  bool contains(const T &value) noexcept {
    Node<T> *curr = head;
    while (curr) {
      if (curr->data == value) {
        return true;
      }
      curr = curr->next;
    }
    return false;
  }

  bool containsElementOf(LinkedList<T> *otherList) noexcept {
    for (auto &item : *this) {
      if (otherList->contains(item)) {
        return true;
      }
    }
    return false;
  }

  T &get(int index) noexcept {
    Node<T> *curr = head;
    for (int i = 0; i < index; i++) {
      curr = curr->next;
      assert(curr && "Index out of bounds.");
    }
    return curr->data;
  }

  T &operator[](int index) noexcept { return get(index); }

  size_t size() noexcept {
    size_t count = 0;
    Node<T> *curr = head;
    while (curr) {
      count++;
      curr = curr->next;
    }
    return count;
  }

  bool empty() noexcept { return head == nullptr; }

  class Iterator {
  private:
    Node<T> *current;

  public:
    Iterator(Node<T> *head) noexcept : current(head) {}

    T &operator*() noexcept { return current->data; }
    T *operator->() noexcept { return &current->data; }
    bool operator==(const Iterator &other) noexcept {
      return current == other.current;
    }
    bool operator!=(const Iterator &other) noexcept {
      return current != other.current;
    }

    Iterator &operator++() noexcept {
      current = current->next;
      return *this;
    }
  };

  Iterator begin() noexcept { return Iterator(head); }
  Iterator end() noexcept { return Iterator(nullptr); }

#ifdef DEBUG
  void print() noexcept {
    if (!head) {
      std::cout << "[]";
      return;
    }
    Node<T> *curr = head;
    while (curr) {
      std::cout << (int)curr->data << " ";
      curr = curr->next;
    }
  }
#endif
};

using Id = int;

class IdList : public LinkedList<Id> {
public:
  IdList() noexcept : LinkedList<Id>() {
    std::memset(this, 0, sizeof(IdList));
  }

  IdList(const Id &value) noexcept : LinkedList<Id>(){
    head = Node<Id>::create(value);
  }

  IdList(const IdList &other) noexcept : LinkedList<Id>() {
    Node<Id> *curr = other.head;
    while (curr) {
      push_back(curr->data);
      curr = curr->next;
    }
  }

  // Move constructor
  IdList(IdList &&other) noexcept : LinkedList<Id>() {
    head = other.head;
    other.head = nullptr;
  }

  IdList &operator=(const IdList &other) noexcept {
    if (this != &other) {
      clear();
      Node<Id> *curr = other.head;
      while (curr) {
        push_back(curr->data);
        curr = curr->next;
      }
    }
    return *this;
  }

  // Move assignment
  IdList &operator=(IdList &&other) noexcept {
    if (this != &other) {
      clear();
      head = other.head;
    }
    other.head = nullptr;

    return *this;
  }

};
