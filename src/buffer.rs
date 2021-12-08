use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Result};



/**
 * Avaiable only when to_read is 0;
 */
pub struct BData<T: Copy + Display + Default>
{
    pub to_read: isize,
    pub data: Option<T>,
}

impl<T: Copy + Display + Default> BData<T>
{
    pub fn _locked(&self) -> bool
    {
        self.to_read <= 0
    }
}

impl<T: Copy + Display + Default> Display for BData<T>
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let data = self.data.unwrap_or_default();
        write!(f, "{{to_read: {}, data: {}}}", self.to_read, data)
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
pub struct SBuffer<T: Copy + Display + Default>
{
    numpos: usize,
    numprod: usize,
    numcons: usize,

    free_slots: usize,
    nxt_free: VecDeque<usize>,
    nxt_data: VecDeque<VecDeque<usize>>,

    data: Vec<BData<T>>
}

impl<T: Copy + Display + Default> Display for SBuffer<T>
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.

        // printf("%d::%d -> Buffer[ ", tid, data);
        // for(int i = 0; i < args->buffer->numpos; i++)
        // {
        //     char str[10];
        //     sprintf(str, "%d", args->buffer->data[i]);
        //     char* symbol = args->buffer->data[i] >= 0 ? str : "*";
        //     printf("%s ", symbol);
        // }
        // printf(" ] ( free slots: %d next free: %d )\n", args->buffer->free_slots, front(args->buffer->nxt_free));

        write!(f, "Buffer[ ");
        for d in &self.data
        {
            write!(f, "{}; ", d.data.unwrap_or_default());
        }
        write!(f, " ] free_slots: {} next_free: {}", self.free_slots, self.nxt_free.back().unwrap())
    }
}

impl<T: Copy + Display + Default> SBuffer<T>
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
            sb.nxt_data.push_front(VecDeque::with_capacity(numpos));
        }
        sb
    }
}

impl<T: Copy + Display + Default> SBuffer<T>
{
    pub fn data(&self, index: usize) -> Option<T>
    {
        self.data[index].data
    }

    pub fn len(&self) -> usize
    {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool
    {
        let mut empty = true;
        for d in   &(self.data)
        {
            if !(d.data.is_none())
            {
                empty = false;
            } 
        }
        empty
    }

    pub fn numpos(&self) -> usize {self.numpos} 
    pub fn numprod(&self) -> usize {self.numprod}
    pub fn numcons(&self) -> usize {self.numcons}
    pub fn free_slots(&self) -> usize {self.free_slots}
}

impl<T: Copy + Display + Default> SBuffer<T>
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
                self.nxt_data.get_mut(i).unwrap().push_front(idx);
            }
            //     buffer->data[idx] = item;
            self.data.get_mut(idx as usize).unwrap().data = Some(item);
            //     buffer->to_read[idx] = buffer->numcons;
            self.data.get_mut(idx as usize).unwrap().to_read = self.numcons as isize;
            //     buffer->free_slots--;
            self.free_slots -= 1;
        }

    }

    pub fn pop(&mut self, my_id: usize) -> Option<T>
    {
        // int data = -1;
        let mut data: Option<T> = None;
        // if(buffer->free_slots < buffer->numpos && !isEmpty(buffer->nxtdata[meuid]))
        if self.free_slots < self.numpos && !(self.nxt_data[my_id].is_empty())
        {
            //     int idx = dequeue(buffer->nxtdata[meuid]);
            let idx = self.nxt_data[my_id].pop_back().unwrap();
            //     data = buffer->data[idx];
            data = self.data.get(idx).unwrap().data;
            //     (buffer->to_read[idx])--;
            (&mut self.data[idx]).to_read -= 1;
            //     if(buffer->to_read[idx] == 0)
            if self.data[idx].to_read == 0
            {
            //         buffer->free_slots++;
                self.free_slots += 1;
            //         buffer->data[idx] = -1;
                (&mut self.data[idx]).data = None;
            //         buffer->to_read[idx] = -1;
                (&mut self.data[idx]).to_read = -1;
            //         enqueue(buffer->nxt_free, idx);
                self.nxt_free.push_front(idx);
            }
        }
        data
        // return data;
    }
}