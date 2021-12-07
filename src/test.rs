

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use crate::buffer::{BData, SBuffer};


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
        assert_eq!(buffer.numpos, capacity);
        assert_eq!(buffer.numcons, 2);
        assert_eq!(buffer.numprod, 2);
        assert_eq!(buffer.nxt_free.len(), 10);
        assert_eq!(buffer.nxt_data.len(), 2);
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
        assert_eq!(buffer.data[0].data.unwrap(), 100);
        assert_eq!(buffer.data[1].data.unwrap(), 200);
        assert_eq!(buffer.data[2].data.unwrap(), 300);
    }
}