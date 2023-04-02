# FunkyHW
FunkyHW is a program to write Hello & World to stdout from two separate threads alternatively.

### Usage of channels and threads
To achieve this, I have used std::mpsc::channel and std::thread.

#### How
Here to make this alternating behaviour between threads, I have initialized two channel pairs:
```
let (tx1, rx1) = sync::mpsc::channel();
```

```
let (tx2, rx2) = sync::mpsc::channel();
```

After these two initializations, we initialize two threads and moving the channels in X configuration:
```
let thread_1 = thread::spawn(move || loop {
    if rx1.recv().unwrap() == 1 {
        // -- snip
        tx2.send(2).unwrap();
    }
})
```
```
let thread_2 = thread::spawn(move || loop {
    if rx2.recv().unwrap() == 2 {
        // -- snip
        tx1.send(1).unwrap();
    }
})
```
<br />
Here we used the sender and receiver channel pair in separate threads, so that one sender from a separate thread when sends the message, the receiver pair will receive the message in another thread, and on equating the value the loop will run.
<br />
- The loops will constantly run, but if the Receiver channel didn't received the message, the loop on the thread will keep running until the Sender channel sends the specific message from another thread, and after receiving the message the Receiver channel buffer will become empty and the if condition will not be satisfied after consumption of one message. 
<br />
- The Sender channel is asynchronous, so all send() calls will be non-blocking.
<br />
After defining threads, we will have to initiate the X thread calling by sending `1` to the `rx1` so the `if` condition in `thread_1` will satisfy which will then send `2` to `rx2` upon which the `if` condition in `thread_2` will be satisfied and then the `thread_2` will send `1` to `thread_1` to satisfy the if condition, and so on.
```
let tx = sync::mpsc::Sender::clone(tx1);

tx.send(1).unwrap();
```
<br />
Hence making threads act alternatively and print the required message of "Hello" from `thread_1` and "World" from `thread_2` alternatively.
