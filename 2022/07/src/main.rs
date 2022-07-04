use std::marker::PhantomPinned;
use std::pin::Pin;

struct NotUnpin {
    _pinned: PhantomPinned,
    x: i32
}

impl NotUnpin {
    pub fn new(x: i32) -> NotUnpin {
        NotUnpin {
            _pinned: PhantomPinned,
            x: x
        }
    }

    pub fn method1(self: &Self) {
        println!("x:{} ", self.x);
    }

    pub fn method2(self: &mut Self, new_x: i32) {
        self.x = new_x;
    }

    pub fn method3(self: Pin<&mut Self>, new_x: i32) {
        unsafe {
            let this: &mut Self = self.get_unchecked_mut();
            this.x = new_x;
        }
    }
}

fn main() {
    let obj1 = NotUnpin::new(1);
    let obj2 = NotUnpin::new(2);

    let mut obj1: Pin<Box<NotUnpin>> = Box::pin(obj1);
    let mut obj2:     Box<NotUnpin>  = Box::new(obj2);

    //let _ = std::mem::replace(&mut *obj1, NotUnpin::new(3)); //error
    let _ = std::mem::replace(&mut *obj2, NotUnpin::new(3));

    obj1.as_ref().method1();
    obj2.as_ref().method1();
    obj1.as_mut().method3(21);
    obj2.as_mut().method2(22);
    obj1.as_ref().method1();
    obj2.as_ref().method1();
}
