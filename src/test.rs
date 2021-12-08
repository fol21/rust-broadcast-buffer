


#[cfg(test)]
mod tests {
    use std::sync::{Mutex, Arc};
    use std::thread;
    use rand;
    use crate::buffer::{SBuffer};


    #[test]
    /**
     * Creates a i32 SBuffer with:
     *  {capacity} positions;
     *  1 producer slot;
     *  1 consumer slot;
     * 
     * For filling with BData.
     */
    fn test_create_buffer() {
        let numprod = 2;
        let numcons = 2;
        let capacity = 10;
        let buffer: SBuffer<i32> = SBuffer::with_capacity(capacity, numprod, numcons);
        assert_eq!(buffer.numpos(), capacity);
        assert_eq!(buffer.numcons(), 2);
        assert_eq!(buffer.numprod(), 2);
    }

    #[test]
    /** Push to Buffer 
     * 
    */
    fn test_push_to_buffer() {
        let numprod = 2;
        let numcons = 2;
        let capacity = 10;
        let mut buffer: SBuffer<i32> = SBuffer::with_capacity(capacity, numprod, numcons);
        buffer.push(100);
        buffer.push(200);
        buffer.push(300);
        assert_eq!(buffer.data(0).unwrap(), 100);
        assert_eq!(buffer.data(1).unwrap(), 200);
        assert_eq!(buffer.data(2).unwrap(), 300);
    }

    #[test]
    /** Pop from Buffer 
     * 
    */
    fn test_pop_from_buffer() {
        let numprod = 4;
        let numcons = 2;
        let insertions = 2;
        let capacity = numprod * insertions;
        let mut buffer: SBuffer<i32> = SBuffer::with_capacity(capacity, numprod, numcons);

        for i in 0..capacity
        {
            buffer.push(((i+1) * 100) as i32);
        }

        let mut data;
        for i in 0..capacity
        {
            for c in 0..numcons
            {
                data = buffer.pop(c);
                assert_eq!(data.unwrap(), ((i+1) * 100) as i32);
            }
        }
        assert_eq!(buffer.is_empty(), true);
    }

    #[test]
    fn test_deposit()
    {
        let data = rand::random::<i32>();
        let shared_buffer: Arc<Mutex<SBuffer<i32>>> = Arc::new(Mutex::new(SBuffer::with_capacity(10, 1, 1)));
        let my_buffer = Arc::clone(&shared_buffer);
        let handle = thread::spawn(move || {
            let mut buff = my_buffer.lock().unwrap();
            buff.push(data);
        });
        handle.join().unwrap();
        assert_eq!(shared_buffer.lock().unwrap().data(0).unwrap(), data);
    }

    #[test]
    fn test_consume()
    {
        let shared_buffer: Arc<Mutex<SBuffer<i32>>> = Arc::new(Mutex::new(SBuffer::with_capacity(10, 1, 1)));
        shared_buffer.lock().unwrap().push(100);
        shared_buffer.lock().unwrap().push(200);
        
        let my_buffer = Arc::clone(&shared_buffer);
        let handle = thread::spawn(move || -> Option<i32> {
            let mut buff = my_buffer.lock().unwrap();
            buff.pop(0)
        });
        let res = handle.join().unwrap();
        assert_eq!(res.unwrap(), 100);
    }
}