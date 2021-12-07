
use std::sync::{Mutex, Arc};
use std::thread;

mod buffer;
use buffer::{BData, SBuffer};

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
// fn deposita_handler(buffer: SBuffer<i32>, insertions: i32)
// {

// }

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

fn main() {
    let shared_buffer: Arc<Mutex<SBuffer<u32>>> = Arc::new(Mutex::new(SBuffer::with_capacity(10, 1, 1)));
    let mut handles = vec![];

    for n in 0..10
    {
        let my_buffer = Arc::clone(&shared_buffer);
        let handle = thread::spawn(move || {
            let mut buff = my_buffer.lock().unwrap();
            buff.push(n);
            // buff.data.push(BData{to_read: 0, data: n});
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let end = shared_buffer.lock().unwrap().data.len() - 1;
    for i in 0..end
    {
        println!("Result: {}", shared_buffer.lock().unwrap().data[i].data.unwrap());
    }
    
}
