# demangle

A tool to demangle symbol in rust linux kernel module stack backtrace when segment fault

## Usage

```
[w@ww linux]$ demangle _RNvXs9_Csa5tTp5JGY9w_14r4l_e1000_demoNtB5_9NetDeviceNtNtCsfATHBUcknU9_6kernel3net16DeviceOperations4open
<r4l_e1000_demo[75818f0ee37687e0]::NetDevice as kernel[b5a56d23ad401ad7]::net::DeviceOperations>::open
```

## TODO
add cpp demangle example?

## rust kernel module backtrace example

```
~ # ip link set eth0 up
[   86.105522] r4l_e1000_demo: [/home/w/repos/learningOS/stage3_homework/src_e1000/r4l_e1000_demo.rs:154] "--> open" = "--> open"
[   86.106090] r4l_e1000_demo: Rust for linux e1000 driver demo (net device open)
[   86.106491] r4l_e1000_demo: [/home/w/repos/learningOS/stage3_homework/src_e1000/r4l_e1000_demo.rs:158] line!() = 158
[   86.106974] r4l_e1000_demo: [/home/w/repos/learningOS/stage3_homework/src_e1000/r4l_e1000_demo.rs:162] line!() = 162
[   86.107654] r4l_e1000_demo: [/home/w/repos/learningOS/stage3_homework/src_e1000/r4l_e1000_demo.rs:164] line!() = 164
[   86.108153] r4l_e1000_demo: [/home/w/repos/learningOS/stage3_homework/src_e1000/r4l_e1000_demo.rs:172] line!() = 172
[   86.108687] r4l_e1000_demo: [/home/w/repos/learningOS/stage3_homework/src_e1000/r4l_e1000_demo.rs:175] line!() = 175
[   86.109173] r4l_e1000_demo: [/home/w/repos/learningOS/stage3_homework/src_e1000/r4l_e1000_demo.rs:177] line!() = 177
[   86.109741] r4l_e1000_demo: [/home/w/repos/learningOS/stage3_homework/src_e1000/r4l_e1000_demo.rs:183] line!() = 183
[   86.110455] r4l_e1000_demo: [/home/w/repos/learningOS/stage3_homework/src_e1000/r4l_e1000_demo.rs:189] line!() = 189
[   86.110982] r4l_e1000_demo: [/home/w/repos/learningOS/stage3_homework/src_e1000/r4l_e1000_demo.rs:191] line!() = 191
[   86.111776] ------------[ cut here ]------------
[   86.112038] kernel BUG at net/core/dev.c:6438!
[   86.112918] invalid opcode: 0000 [#1] PREEMPT SMP NOPTI
[   86.113455] CPU: 0 PID: 85 Comm: ip Tainted: G           O       6.1.0-rc1 #
[   86.113899] Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS Arch Linux 1.16.2-1-1 04/01/2014
[   86.114511] RIP: 0010:napi_enable+0x51/0x60
[   86.115236] Code: 8b 77 38 f6 86 61 08 00 00 02 74 d0 48 89 d6 48 81 ce 00 01 00 00 48 83 bf 88 01 00 00 00 48 0f 45 d6 eb b8 58 c3 cc cc cc cc <0f> 0b8
[   86.116162] RSP: 0018:ffffb5de801d7a08 EFLAGS: 00000246
[   86.116456] RAX: 0000000000000010 RBX: ffff9c9242804500 RCX: 0000000000000010
[   86.116794] RDX: 0000000000000000 RSI: c0000000ffffdfff RDI: ffff9c924123a408
[   86.117133] RBP: ffffb5de801d7aa8 R08: 0000000000000000 R09: ffffffffbb081880
[   86.117464] R10: 00000000ffffffff R11: 00000000ffffdfff R12: ffff9c9241ca5fe0
[   86.117808] R13: 000000000000000d R14: ffffb5de801d7a18 R15: ffffb5de801d7ad0
[   86.118215] FS:  0000000000e4c3c0(0000) GS:ffff9c9247800000(0000) knlGS:0000000000000000
[   86.118661] CS:  0010 DS: 0000 ES: 0000 CR0: 0000000080050033
[   86.118952] CR2: 000000000053ca5b CR3: 0000000002826000 CR4: 00000000000006f0
[   86.119391] Call Trace:
[   86.120226]  <TASK>
[   86.120740]  _RNvXs9_Csa5tTp5JGY9w_14r4l_e1000_demoNtB5_9NetDeviceNtNtCsfATHBUcknU9_6kernel3net16DeviceOperations4open+0x1b84/0x1e20 [r4l_e1000_demo]
[   86.121870]  ? _RNvXsR_NtCs3yuwAp0waWO_4core3fmtRNtNtCsfATHBUcknU9_6kernel3str4CStrNtB5_7Display3fmtCsa5tTp5JGY9w_14r4l_e1000_demo+0x20/0x20 [r4l_e1000_]
[   86.122464]  ? _RNvXs2_NtNtNtCs3yuwAp0waWO_4core3fmt3num3implNtB9_7Display3fmt+0x20/0x20
[   86.122809]  ? _RNvXsR_NtCs3yuwAp0waWO_4core3fmtRNtNtCsfATHBUcknU9_6kernel3str4CStrNtB5_7Display3fmtCsa5tTp5JGY9w_14r4l_e1000_demo+0x20/0x20 [r4l_e1000_]
[   86.123378]  ? _RNvXsP_NtCs3yuwAp0waWO_4core3fmtRRuNtB5_5Debug3fmtCsa5tTp5JGY9w_14r4l_e1000_demo+0x20/0x20 [r4l_e1000_demo]
[   86.123867]  ? _RNvXsP_NtCs3yuwAp0waWO_4core3fmtRmNtB5_5Debug3fmtCsa5tTp5JGY9w_14r4l_e1000_demo+0x50/0x50 [r4l_e1000_demo]
[   86.124353]  ? _RNvMs2_NtCsfATHBUcknU9_6kernel3netINtB5_12RegistrationNtCsa5tTp5JGY9w_14r4l_e1000_demo9NetDeviceE13open_callbackBS_+0x29/0x50 [r4l_e1000]
[   86.124941]  ? __dev_open+0x122/0x1e0
[   86.125114]  ? __dev_change_flags+0xaa/0x1c0
[   86.125314]  ? dev_change_flags+0x1c/0x60
[   86.125494]  ? devinet_ioctl+0x4ae/0x5a0
[   86.125680]  ? inet_ioctl+0x19c/0x240
[   86.125861]  ? sock_do_ioctl+0x67/0x1a0
[   86.126043]  ? sock_ioctl+0x267/0x350
[   86.126217]  ? __se_sys_ioctl+0x6d/0xb0
[   86.126401]  ? do_syscall_64+0x43/0x90
[   86.126578]  ? entry_SYSCALL_64_after_hwframe+0x63/0xcd
[   86.126844]  </TASK>
[   86.127020] Modules linked in: r4l_e1000_demo(O)
[   86.127826] ---[ end trace 0000000000000000 ]---
[   86.128102] RIP: 0010:napi_enable+0x51/0x60
[   86.128323] Code: 8b 77 38 f6 86 61 08 00 00 02 74 d0 48 89 d6 48 81 ce 00 01 00 00 48 83 bf 88 01 00 00 00 48 0f 45 d6 eb b8 58 c3 cc cc cc cc <0f> 0b 8
[   86.129157] RSP: 0018:ffffb5de801d7a08 EFLAGS: 00000246
[   86.129455] RAX: 0000000000000010 RBX: ffff9c9242804500 RCX: 0000000000000010
[   86.129791] RDX: 0000000000000000 RSI: c0000000ffffdfff RDI: ffff9c924123a408
[   86.130121] RBP: ffffb5de801d7aa8 R08: 0000000000000000 R09: ffffffffbb081880
[   86.130487] R10: 00000000ffffffff R11: 00000000ffffdfff R12: ffff9c9241ca5fe0
[   86.130823] R13: 000000000000000d R14: ffffb5de801d7a18 R15: ffffb5de801d7ad0
[   86.131161] FS:  0000000000e4c3c0(0000) GS:ffff9c9247800000(0000) knlGS:0000000000000000
[   86.131580] CS:  0010 DS: 0000 ES: 0000 CR0: 0000000080050033
[   86.131852] CR2: 000000000053ca5b CR3: 0000000002826000 CR4: 00000000000006f0
Segmentation fault
```
