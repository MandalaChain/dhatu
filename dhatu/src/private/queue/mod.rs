use std::{collections::VecDeque, sync::Arc};

use tokio::sync::Mutex;

type InnerPool<T> = VecDeque<T>;

pub struct ThreadSafeQueue<T> {
    inner: Arc<Mutex<InnerPool<T>>>,
}

impl<T> Clone for ThreadSafeQueue<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> ThreadSafeQueue<T> {
    pub fn new() -> Self {
        let inner = VecDeque::<T>::new();
        let inner = Arc::new(Mutex::new(inner));

        Self { inner }
    }

    pub async fn get(&mut self) -> Option<T> {
        let mut inner = self.inner.lock().await;
        inner.pop_front()
    }

    pub async fn add(&mut self, data: T) {
        let mut inner = self.inner.lock().await;
        inner.push_back(data)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[actix::test]
//     async fn should_queue_correctly() {
//         let mut queue = ThreadSafeQueue::new();

//         let expected_first_batch = vec![1];
//         let expected_second_batch = vec![2];
//         let expected_third_batch = vec![3];

//         queue.add(expected_first_batch.clone()).await;
//         queue.add(expected_second_batch.clone()).await;
//         queue.add(expected_third_batch.clone()).await;

//         let actual_first_batch = queue.get().await.unwrap();
//         let actual_second_batch = queue.get().await.unwrap();
//         let actual_third_batch = queue.get().await.unwrap();

//         assert_eq!(expected_first_batch[0], actual_first_batch[0]);
//         assert_eq!(expected_second_batch[0], actual_second_batch[0]);
//         assert_eq!(expected_third_batch[0], actual_third_batch[0]);

//         let batch = queue.get().await;

//         assert!(batch.is_none());
//     }
// }
