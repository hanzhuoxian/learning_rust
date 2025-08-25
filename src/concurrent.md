# 并发

并发：代表程序的不同部分互相独立的运行。
并行：代表程序的不同部分同时运行。


## 线程

```rust
use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    
    for i in 1..5 {
        println!("hi number {i} from the mail thread!");
        thread::sleep(Duration::from_millis(1));
    }

}

```


## 多生产者 channel

```rust
use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move ||{
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thead"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


      thread::spawn(move ||{
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

## 共享状态并发

### 互斥器

互斥器（mutex）是 mutual exclusion 的缩写，也就是说，任意时刻，其只允许一个线程访问某些数据。
为了访问互斥器中的数据，线程首先需要通过获取互斥器的 锁（lock）来表明其希
望访问数据。
锁是一个作为互斥器一部分的数据结构，它记录谁有数据的排他访问权。因此，我们描述互斥器为通过锁系统 保护（guarding）其数据。

### Arc

原子引用计数，线程安全的。线程安全带有性能惩罚。所以在单线程环境中 Rc 是符合需求且最高效的。