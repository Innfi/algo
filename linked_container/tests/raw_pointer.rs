// but am i ever get a chance to use this?
#[test]
fn test_init_pointer() {
  let mut origin: i32 = 10;
  let ptr_origin: *const i32 = &origin;

  unsafe {
    assert_eq!(*ptr_origin, 10);
  }

  let ptr_mut_origin: *mut i32 = &mut origin;
  unsafe {
    *ptr_mut_origin = 20;
    assert_eq!(origin, 20);
  }
}