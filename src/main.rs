use std::ops::{Deref, DerefMut};

struct MyBox<T, U>(T, U);

impl<T, U> MyBox<T, U> {
    fn new(x: T, y: U) -> MyBox<T, U> {
        MyBox(x, y)
    }
}

impl<T, U> Deref for MyBox<T, U> {
    type Target = U;//这个Self::Target 也影响规定了兄弟方法DerefMut中的Target

    fn deref(&self) -> &Self::Target {
        &self.1
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
    let y = MyBox::new(0, x);
    assert_eq!(5, x);
    assert_eq!(5, *y);// * 只是语法糖   *x  等价于  *(x.deref())
    // & 和 *  的优先级 比 . 低，但比别的一元 操作符 高，不放心优先级就加()
    assert_eq!(5, *y.deref());
    // rust 里面无脑 引用 借用 使用 &符号
    let m = MyBox::new(11, String::from("Rust"));
    hello(&m);
    hello(&String::from("xxxx"));


    /*
    rust 所有权 与 借用规则

    -1. 资源寿命 比 资源所有者 长， 资源所有者寿命比 借入者寿命 长
    0. 一旦失去所有权( 比如资源没有实现 copy 特性时 ，原先所有者被 let 给了另一个变量)，就不能在后面代码里出现了，领盒饭
    1. 可变借用与所有者之间都是不共享的，同一时间只有一方能改资源，同时配合编译器自动智能归还可变借用给所有者
    2. 不可变的共享借用可以多次出现


    代码常见套路是
                                                         / 多个只读借用者 持有的是 可变权限使用者 而不是 那个 资源
                                                        /
    <资源唯一的可变权限使用者（所有权拥有者 或者 拿到可变借用者）>  --- 多个只读借用者 持有的是 可变权限使用者 而不是 那个 资源
                                                        \
                                                         \ 多个只读借用者 持有的是 可变权限使用者 而不是 那个 资源

    这样就
    既保证了 多个只读者 同时借助 唯一的资源可变者 的手  间接地  资源 进行 读取，
    又保证了 资源的 修改和释放 没有 副作用，不会内存泄露， 一旦 唯一资源所有者被释放，资源也就会被释放

    */


    let s = String::from("StringResource");
    let v = &mut MyBox(22, s);// 这里s所有权被 v.1 抢走了，后面代码不能出现s
    hello_mut(v);
    hello_mut(v);

    let vv = &mut v.1;// 资源所有权在v.1手上，vv向v.1获取可变借用
    hello_mut(v);
    //hello_mut(vv);
    //当上面这句没有被注释时，可变不共享原则，由于v.1资源已经被vv可变借用，所以即使是拥有者v.1也不能改，所以上面这句注释的语句不能通过编译
    hello_mut(v);//如果某行后面某个借用变量不再出现，rust编译器会智能归还借用，于是下面v.1又可以用


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


    /*
    output

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
}
