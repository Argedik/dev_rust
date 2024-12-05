use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

fn main (){
  deadlock();
}

struct Account {
  owner: String,
  balance: f32,
}

pub fn deadlock() {
  let my_account = Arc::new(Mutex::new(Account {
    owner: "John Doe".to_string(),
    balance: 200.0,
  }));
  let other_account = Arc::new(Mutex::new(Account {
    owner: "John Doe".to_string(),
    balance: 200.0,
  }));

  let my_account_clone = Arc::clone(&my_account);
  let other_account_clone = Arc::clone(&other_account);

  let handle_1 = thread::spawn(move || {
    let mut source_account  = my_account_clone.lock().unwrap();
    println!("Thread 1 : Locked by source account");
    thread::sleep(Duration::from_secs(2));
    let mut target_account  = other_account_clone.lock().unwrap();
    println!("Thread 1 : Locked by target account");
    source_account.balance -= 50.0;
    target_account.balance += 50.0;
  });

  let my_account_clone : Arc<Mutex<Account>> = Arc::clone(&my_account);
  let other_account_clone : Arc<Mutex<Account>> = Arc::clone(&other_account);

  let handle_2 = thread::spawn(move || {
    let mut source_account = my_account_clone.lock().unwrap();
    println!("Thread 2 : Locked by source account");
    thread::sleep(Duration::from_secs(2));
    let mut target_account = other_account_clone.lock().unwrap();
    println!("Thread 2 : Locked by target account");

    target_account.balance -= 25.0;
    source_account.balance += 25.0;
  });

  handle_1.join().unwrap();
  handle_2.join().unwrap();

  println!(
    "Final balances My Account {}, Other Account {}",
    my_account.lock().unwrap().balance,
    other_account.lock().unwrap().balance
  )
}

pub fn poisoning() {
  let log_file = File::create("system.log").expect("Unable to create file");
  let log_file = Arc::new(Mutex::new(log_file));

  let log_file_clone = Arc::clone(&log_file);
  let handle = thread::spawn(move || {
      let mut file = log_file_clone.lock().unwrap();
      writeln!(file, "Thread 1: Writing the system health status").unwrap();
      panic!("Errors occurred while writing to the log file!");
  });

  let log_file_clone = Arc::clone(&log_file);
  let recovery_thread = thread::spawn(move || {
      let mut file = log_file_clone.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
      thread::sleep(Duration::from_secs(3));
      writeln!(file, "Thread 2: Recovering from poisoned state...").unwrap();
  });

  handle.join().unwrap();
  recovery_thread.join().unwrap();

  println!("Log file operations completed");
}


deadlock olma problemi
Şu anda her iki thread birbirini kitlediği için  mutexguard larıyla birbirlerini bekliyorlar o yüzden işlem sonlanmıyor.
Burada çözüm ne olabilir deadlock'a düşmemek için 
Thread lerin aslında kullandıkları mutexguard larda bunların aynı sırada çalıştırılması öneriliyor.
Yani ilk thread önce my_account_clone u daha sonra other_account_clone u kilitleyerek işlem yaparken, diğeri önce target_account u kilitleyerek başladı bu sürece. Bunun sıralı yapılması öneriliyor. 
Burada gerçekten sıralı işletirsek thread içindeki mutexguardları sorun çözülür mü 

