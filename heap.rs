/// heap sort 
pub fn heap_sort(items: &mut [usize]){
    let mut n  = items.len();
    let mut i = n/2;
    while i>= 1 {
        sink(items, i, n);
        i-=1;
    }

    while n>1 {
        exch(items, 1, n);
        n-=1;
        sink(items, 1, n);
    }
}

fn sink(items: &mut [usize], k: usize, n: usize) {
    let mut j= 0;
    let mut kk = k;
    while 2*kk <= n {
         j = 2*kk;
         if j< n && less(items, j, j+1) {
            j+=1;
         }

         if !less(items, kk, j) {
             break;
         }
         exch(items, kk, j);
         kk = j;
    }
}

fn exch(items: &mut [usize],i:usize ,j:usize){
    items.swap(i-1, j-1);
}

fn less(items: &mut [usize],i:usize, j: usize) -> bool {
    return  items[i-1] < items[j-1];
}