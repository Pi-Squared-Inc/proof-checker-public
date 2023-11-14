#include "data_structures.hpp"
#include <cstdlib>

template <typename T> class Rc {
public:
  Rc() noexcept : ptr(nullptr), ref_count(nullptr) {}

  Rc(T *raw_ptr) noexcept : ptr(raw_ptr), ref_count(nullptr) {
    ref_count = static_cast<int *>(
        malloc(sizeof(int))); // Initialize the reference count to 1
    *ref_count = 1;
  }

  Rc(const Rc &other) noexcept : ptr(other.ptr), ref_count(other.ref_count) {
    if (ref_count) {
      (*ref_count)++; // Increment the reference count
    }
  }

  ~Rc() noexcept {
    if (ref_count) {
      (*ref_count)--; // Decrement the reference count
      if (*ref_count == 0) {

        if (ptr) {
          ptr->~T(); // Call the destructor
          free(ptr);
        }
        free(ref_count);
      }
    }
  }

  Rc &operator=(const Rc &other) noexcept {
    if (this != &other) {
      if (ref_count && (*ref_count) > 0) {
        (*ref_count)--;
        if (*ref_count == 0) {
          ptr->~T();
          free(ptr);
          free(ref_count);
        }
      }

      ptr = other.ptr;
      ref_count = other.ref_count;

      if (ref_count) {
        (*ref_count)++;
      }
    }
    return *this;
  }

  bool operator==(const Rc &rhs) const noexcept {
    if (!ptr && !rhs.ptr) {
      return true;
    } else if (!ptr || !rhs.ptr) {
      return false;
    }
    return ptr->operator==(*rhs.ptr);
  }

  T &operator*() noexcept { return *ptr; }

  T *operator->() noexcept { return ptr; }

  void release() noexcept {
    if (ref_count && (*ref_count) > 0) {
      (*ref_count)--;
      if (*ref_count == 0) {
        ptr->~T();
        free(ptr);
        free(ref_count);
      }
    }
  }

  Rc clone() noexcept { return Rc(*this); }

private:
  T *ptr = nullptr;
  int *ref_count = nullptr;
};
