# Конвертор файлов а .tap

## Структура
Весь файл состоит из блоков следующей структуры:
```rust
struct tap_data<const SIZE: usize> {
    header1: u32,
    header2: u32,
    data: [SIZE],
}
```
header1 == header2 == SIZE, где самый старший бит обозначает, что дынные в блоке битые.
