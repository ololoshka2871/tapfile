# Конвертор файлов в .tap
Для работы с [Pertec-Interface-Tape-Controller](https://github.com/ololoshka2871/Pertec-Interface-Tape-Controller)

## Структура
Весь файл состоит из блоков следующей структуры:
```rust
struct tap_data<const SIZE: usize> {
    header1: u32,
    data: [SIZE],
    header2: u32,
}
```
header1 == header2 == SIZE, где самый старший бит обозначает, что данные в блоке битые.

В конце файла записывается 0xFFFFFFFF, что означает конец образа.

## Примеры
- Упаковать файл/данные в .tap:
```bash
./any2tap -b 65535 file.bin file.tap
echo "some data" | ./any2tap file.tap
echo "more data" | ./any2tap -b 65535 > file.tap
tar -cf- files1 file2 file3 | ./any2tap -b 65535 file.tap
```

- Распаковать файл/данные из .tap:
```bash
./tap2any file.tap file.bin
./tap2any file.tap > file.bin
cat file.tap | ./tap2any | tar -xf-
```
