## Experiment 1.2: Understanding how it works

![img.png](img.png)

Ketika di-run, "hey hey" ke-print sebelum "howdy!".

Spawner.spawn nge-package blok asynchronous jadi task dan menaruhnya ke channel queue tapi tidak langsung tereksekusi.

Main thread-nya lanjut dan mengeksekusi println!("... hey hey").

Executor.run() dipanggil, yang langsung nge-pull task-task dari queue dan dikumpulin, sehingga keprint "howdy!", timer delay, dan akhirnya "done!".
