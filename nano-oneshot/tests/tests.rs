use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

use nano_oneshot::{self, RecvError, RecvTimeoutError, SendError};

#[test]
fn oneshot() {
    let (s, r) = nano_oneshot::channel();
    s.send(128).expect("send");
    assert_eq!(r.recv().expect("recv"), 128);
}

#[test]
fn oneshot_send_drop_receiver() {
    let (s, r) = nano_oneshot::channel();
    drop(r);
    assert!(s.is_disconnected());
    assert_eq!(s.send(128).unwrap_err(), SendError::Disconnected(128));
}

#[test]
fn oneshot_recv_drop_sender() {
    let (s, r) = nano_oneshot::channel::<i32>();
    drop(s);
    assert!(r.is_disconnected());
    assert_eq!(r.recv().unwrap_err(), RecvError::Disconnected);
}

#[test]
fn oneshot_recv_timeout_drop_sender() {
    let (s, r) = nano_oneshot::channel::<i32>();
    drop(s);
    assert!(r.is_disconnected());
    assert_eq!(
        r.recv_timeout(Duration::from_secs(1)).unwrap_err(),
        RecvTimeoutError::Disconnected
    );
}

#[test]
fn oneshot_concurrent() {
    let (s, r) = nano_oneshot::channel();

    let handle = thread::spawn(move || s.send(128));

    assert!(handle.join().expect("thread").is_ok());
    assert_eq!(r.recv().expect("recv"), 128);
}

#[test]
fn oneshot_concurrent_timeout() {
    let (s, r) = nano_oneshot::channel();

    let b = Arc::new(Barrier::new(2));

    let handle = {
        let b = Arc::clone(&b);

        thread::spawn(move || {
            b.wait();
            s.send(128)
        })
    };

    assert!(r.recv_timeout(Duration::from_nanos(1)).is_err());
    b.wait();
    assert_eq!(
        handle.join().expect("thread").unwrap_err(),
        SendError::Disconnected(128)
    );
}

#[test]
fn oneshot_concurrent_send_drop_receiver() {
    let (s, r) = nano_oneshot::channel();

    let b = Arc::new(Barrier::new(2));

    let handle = {
        let b = Arc::clone(&b);

        thread::spawn(move || {
            b.wait();
            s.send(128)
        })
    };

    drop(r);
    b.wait();

    assert_eq!(
        handle.join().expect("thread").unwrap_err(),
        SendError::Disconnected(128)
    );
}

#[test]
fn oneshot_concurrent_recv_drop_sender() {
    let (s, r) = nano_oneshot::channel::<i32>();

    let b = Arc::new(Barrier::new(3));
    let b2 = Arc::new(Barrier::new(2));

    let handle = {
        let b = Arc::clone(&b);
        let b2 = Arc::clone(&b2);

        thread::spawn(move || {
            b.wait();
            b2.wait();
            drop(s);
        })
    };

    let handle2 = {
        let b = Arc::clone(&b);

        thread::spawn(move || {
            b.wait();
            r.recv()
        })
    };

    b.wait();
    thread::sleep(Duration::from_millis(5));
    b2.wait();
    handle.join().expect("thread");

    assert_eq!(
        handle2.join().expect("thread").unwrap_err(),
        RecvError::Disconnected
    );
}

#[test]
fn oneshot_concurrent_recv_timeout_drop_sender() {
    let (s, r) = nano_oneshot::channel::<i32>();

    let b = Arc::new(Barrier::new(3));
    let b2 = Arc::new(Barrier::new(2));

    let handle = {
        let b = Arc::clone(&b);
        let b2 = Arc::clone(&b2);

        thread::spawn(move || {
            b.wait();
            b2.wait();
            drop(s);
        })
    };

    let handle2 = {
        let b = Arc::clone(&b);

        thread::spawn(move || {
            b.wait();
            r.recv_timeout(Duration::from_secs(1))
        })
    };

    b.wait();
    thread::sleep(Duration::from_millis(5));
    b2.wait();
    handle.join().expect("thread");

    assert_eq!(
        handle2.join().expect("thread").unwrap_err(),
        RecvTimeoutError::Disconnected
    );
}
