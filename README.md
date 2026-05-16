## Experiment 1.2: Understanding how it works

![img.png](img.png)

Ketika di-run, "hey hey" ke-print sebelum "howdy!".

Spawner.spawn nge-package blok asynchronous jadi task dan menaruhnya ke channel queue tapi tidak langsung tereksekusi.

Main thread-nya lanjut dan mengeksekusi println!("... hey hey").

Executor.run() dipanggil, yang langsung nge-pull task-task dari queue dan dikumpulin, sehingga keprint "howdy!", timer delay, dan akhirnya "done!".

## Experiment 1.3: Multiple Spawn and removing drop

### Before removing drop(spawner)

![img_2.png](img_2.png)

### After removing drop(spawner)

![img_1.png](img_1.png)

Executor-nya didesain untuk terus running selama spawner-nya ada. Kalau drop(spawner) hilang, executor-nya mikir masih ada tasks yang akan ke-spawn, jadi task queue-nya masih open dan nunggu selamanya. Nge-drop spawner akan menutup channel-nya, yang nge-signal ke executor tidak ada lagi task baru yang akan datang, membiarkan programnya ke-exit pas queue saat ini kosong.

## Experiment 2.1: Original code, and how it run

![img_3.png](img_3.png)

Saya run nya dengan membuka 4 terminal, 1 untuk server dan 3 untuk client, saya menegtik 3 message yang berbeda-beda. Ternyata message yang terkirim balik di client 1 adalah semua message dan di client 2 adalah message dari client 2 dan 3, serta di client 3 hanya message yang dikirim client 3.
