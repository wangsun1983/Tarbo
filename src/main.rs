use std::sync::Arc;
use std::sync::Mutex;
use std::rc::Rc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::u32;

use concurrent::container::ConcurrentQueue;
use testsuite::testConcurrent::testCondition;
use testsuite::testConcurrent::testRwlock;
use testsuite::testCoroutine;
use tokio::runtime::Runtime;

use testsuite::testNet::testSocketMonitor;
use testsuite::testConcurrent::testCountDownLatch;
use testsuite::testLang::testSystem;
use testsuite::testConcurrent::testMutex;

mod lang;
mod concurrent;
mod io;
mod testsuite;
mod tools;
mod security;
mod net;
mod coroutine;

fn main() {
    //testSocketMonitor::test_monitor_1();
    //testCoroutine::testFilament::testFilament_1();
    //testCountDownLatch::do_test1();
    //testSystem::test_tid_1();
    //testMutex::test_mutex_1();
    //testMutex::test_mutex_2();
    //testMutex::test_mutex_timeout_1();
    //testCondition::test_condition_1();
    testRwlock::test_rwlock_3();
}
