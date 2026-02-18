# Cenni didattici

- **Runtime async:** Tokio fornisce un executor multi-thread che esegue `Future`. `#[tokio::main]` è macro che crea il runtime e blocca su `main()` async.
- **Task spawning:** `tokio::spawn` lancia un `Future` su un thread del pool. Il future **deve** essere `Send + 'static` — questo è il primo punto dove ownership diventa rilevante: non si possono passare riferimenti non-`'static` a un task spawnato.
- **Ownership tra task:** ogni dato passato a `tokio::spawn` viene **moved** nella closure. Se serve condivisione, si usa `Arc<T>` (e `Mutex` o `RwLock` se serve mutabilità).
- **Bounded vs unbounded channel:** un canale bounded (`mpsc::channel(cap)`) blocca il producer quando il buffer è pieno → backpressure naturale. Un canale unbounded (`mpsc::unbounded_channel`) non blocca mai il send, ma può crescere senza limite → rischio OOM.

> **Punto didattico:** fai notare che `tokio::spawn` richiede `Send + 'static` perché il task può essere eseguito su qualsiasi thread del pool. Questo è diverso da un semplice `.await` che mantiene il future sullo stesso task.