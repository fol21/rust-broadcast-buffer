
use std::sync::{Mutex, Arc};
use std::{thread, time};
use rand;

mod buffer;
use buffer::{SBuffer};

mod test;

// void* deposita_thread(void* arg)
// {
//     pid_t tid = syscall(SYS_gettid);

//     deposita_args* args= (deposita_args*) arg;
//     int count = args->insertions;
//     while (count > 0)
//     {
//         sleep(rand() % 5);
//         int data = args->item * (rand() % 100);     
//         //wait
//         sem_wait(&(args->buffer->mutex));
//         deposita(args->buffer, data);
//         count--;
//         //signal
//         sem_post(&(args->buffer->mutex));
//     }
//     return NULL;
// }
fn deposit_handler(mutex: &mut Arc<Mutex<SBuffer<i32>>>, mut insertions: i32)
{
    while insertions > 0
    {
        thread::sleep(time::Duration::from_millis(rand::random::<u64>() % 5));
        let mut buff = mutex.lock().unwrap();
        buff.push(rand::random::<i32>());
        insertions -= 1;
    }
}

// void* consome_thread(void* arg)
// {
//     pid_t tid = syscall(SYS_gettid);

//     consome_args* args= (consome_args*) arg;
    
//     int count = args->consome;
//     int data[100];
//     int nxt = 0;
//     for (int i = 0; i < 100; i++){ data[i] = -1;}
    
//     while (count-- > 0)
//     {
//         sleep(rand() % 2);
        
//         //wait
//         sem_wait(&(args->buffer->mutex));
//         data[nxt] = consome(args->buffer, args->id);
//         int it = 0;
//         //signal
//         sem_post(&(args->buffer->mutex));
//     } 
//     return NULL;
// }
// fn consome_handler()
// {

// }

fn main()
{
    let numpos = 16;
    let numprod = 2;
    let numcons = 2;

    let shared_buffer: Arc<Mutex<SBuffer<i32>>> = Arc::new(Mutex::new(SBuffer::with_capacity(numpos, numprod, numcons)));
    let mut handles = vec![];

    for _ in 0..numprod
    {
        let mut my_buffer = Arc::clone(&shared_buffer);
        let handle = thread::spawn(move || {
            // let mut buff = my_buffer.lock().unwrap();
            // buff.push(n);
            deposit_handler(&mut my_buffer, 2);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut buff = shared_buffer.lock().unwrap();
    println!("{}", buff);
    // while buff.free_slots() < numpos
    // {
    //     let data = buff.pop(0).unwrap_or_default();
    //     println!("Result: {}, free slots: {}", data, buff.free_slots());
    // }
    
}
