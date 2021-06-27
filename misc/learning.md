### Closures
Closures desugar to structs containing borrows/captures.
Fn / FnMut / FnOnce traits are analogous to methods on types that are &self / &mut self / self.

This requires  FnMut to call because it mutates stuff:

let counter = 0;
move || { counter += 1; }


And would be analogous to:

struct AnonClosure {
  counter: i32,
}

impl FnMut for AnonClosure {
  fn call(&mut self) {
    self.counter += 1;
  }
}

impl Fn would have fn call(&self)
impl FnOnce would have fn call(self)
Oh and that's why sometimes you need Fn + 'static bounds.

Also, every closure/fn implements FnOnce
Because if you can call it many times Fn/FnMut you can definitely call it at least once.
