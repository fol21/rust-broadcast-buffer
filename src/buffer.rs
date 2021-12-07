use std::collections::VecDeque;
use std::fmt::{Display};



/**
 * Avaiable only when to_read is 0;
 */
pub struct BData<T: Copy + Display>
{
    pub to_read: usize,
    pub data: Option<T>,
}

impl<T: Copy + Display> BData<T>
{
    pub fn locked(&self) -> bool
    {
        self.to_read <= 0
    }
}
/** 
 * 
 * struct sbuffer
 * {
 *     i32 numpos;
 *     i32 numprod;
 *     i32 numcons;
 * 
 *     i32 free_slots;
 *     Queue* nxt_free;
 *     Queue** nxtdata;
 *     int* to_read;
 *     
 *     int* data;
 *     sem_t mutex;
 * 
 * }
*/
pub struct SBuffer<T: Copy + Display>
{
    pub numpos: usize,
    pub numprod: usize,
    pub numcons: usize,

    pub free_slots: usize,
    pub nxt_free: VecDeque<usize>,
    pub nxt_data: VecDeque<VecDeque<usize>>,

    pub data: Vec<BData<T>>
}

impl<T: Copy + Display> SBuffer<T>
{
    pub fn with_capacity(numpos: usize, numprod: usize, numcons: usize) -> SBuffer<T>
    {

        let mut sb = SBuffer
        {
            numpos: numpos,
            numprod: numprod,
            numcons: numcons,
            free_slots: numpos,
            nxt_free: VecDeque::with_capacity(numpos),
            nxt_data: VecDeque::with_capacity(numcons),
            data: Vec::with_capacity(numpos)
        };
        for i in 0..(numpos) {
            sb.nxt_free.push_front(i);
            sb.data.push(BData{to_read: 0, data: None});
        }
        for _ in 0..(numcons) {
            sb.nxt_data.push_back(VecDeque::with_capacity(numpos));
        }
        sb
    }
}

impl<T: Copy + Display> SBuffer<T>
{
    pub fn push(&mut self, item: T)
    {
        // if(buffer->free_slots > 0)
        if self.free_slots > 0
        {      
            //     int idx = dequeue(buffer->nxt_free);
            let idx = self.nxt_free.pop_back().unwrap();
            //     for (int i = 0; i < buffer->numcons; i++) {enqueue(buffer->nxtdata[i], idx);}            
            for i in 0..(self.nxt_data.len())
            {
                println!("here {}", i);
                self.nxt_data.get_mut(i).unwrap().push_front(idx);
            }
            //     buffer->data[idx] = item;
            self.data.get_mut(idx as usize).unwrap().data = Some(item);
            println!("{}", self.data.get_mut(idx as usize).unwrap().data.unwrap());
            //     buffer->to_read[idx] = buffer->numcons;
            self.data.get_mut(idx as usize).unwrap().to_read = self.numcons;
            //     buffer->free_slots--;
            self.free_slots -= 1;
        }

    }
}