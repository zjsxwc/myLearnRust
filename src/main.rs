use std::ops::{Deref, DerefMut};
struct MyBox<T, U>(T, U);
impl<T, U> MyBox<T, U> {
    fn new(x: T, y: U) -> MyBox<T, U> {
        MyBox(x, y)
    }
}
impl<T, U> Deref for MyBox<T, U> {
    type Target = U;//这个Self::Target 也影响规定了兄弟方法DerefMut中的Target

    fn deref(&self) -> & Self::Target {
        & self.1
    }
}
impl<T, U> DerefMut for MyBox<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}
fn hello(name: &str) {
    p(name);
}
fn hello_mut(name: &mut String) {
    name.push('!');
    p(name);
}
fn p(name: &str) {
    println!("Hello, {}", name);
}
fn main() {
    let x = 5;
    let y = MyBox::new(0,x);
    assert_eq!(5, x);
    assert_eq!(5, *y);// * 只是语法糖   *x  等价于  *(x.deref())
    // & 和 *  的优先级 比 . 低，但比别的一元 操作符 高，不放心优先级就加()
    assert_eq!(5, *y.deref());
    // rust 里面无脑 引用 借用 使用 &符号
    let m = MyBox::new(11, String::from("Rust"));
    hello(&m);
    hello(&String::from("xxxx"));
    let s = String::from("StringResource");
    let  v = &mut MyBox(22, s);// 这里s所有权被 v.1 抢走了，后面代码不能出现s
    hello_mut(v);
    hello_mut(v);

    let vv = &mut v.1;// 资源所有权在v.1手上，vv向v.1获取可变借用
    hello_mut(vv);
    //hello_mut(v);
    //v.1资源是 可变不共享，由于已经被vv可变借用，所以即使是拥有者v.1也不能改，所以上面这句注释的语句不能通过编译
    hello_mut(vv);//如果某行后面某个借用变量不再出现，rust编译器会智能归还借用，于是下面v.1又可以用


    hello_mut(v);
    let q1 = &v;
    p("q1");
    p(q1);
    hello_mut(v);
    let q2 = &v;
    p("q2");
    p(q2);
    //let q3 = &mut v;
    //q1,q2对于v是共享不可变， 这里q3的写法不能通过编译


    hello_mut(v);
    hello_mut(v);
    hello_mut(v);
    p(v);
}
/**output

/home/wangchao/.cargo/bin/cargo run --color=always --package untitled7 --bin untitled7
   Compiling untitled7 v0.1.0 (/home/wangchao/Downloads/untitled7)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/untitled7`
Hello, Rust
Hello, xxxx
Hello, StringResource!
Hello, StringResource!!
Hello, StringResource!!!
Hello, StringResource!!!!
Hello, StringResource!!!!!
Hello, q1
Hello, StringResource!!!!!
Hello, StringResource!!!!!!
Hello, q2
Hello, StringResource!!!!!!
Hello, StringResource!!!!!!!
Hello, StringResource!!!!!!!!
Hello, StringResource!!!!!!!!!
Hello, StringResource!!!!!!!!!


*/