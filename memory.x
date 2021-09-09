MEMORY
{
/* Reserve 0x19000 (100kB) flash for softdevice */
FLASH : ORIGIN = 0x00019000, LENGTH = 0x67000 /* 512K - 100K */
/* Reserve 0x1ae0 RAM for softdevice */
RAM : ORIGIN = 0x20001ae0, LENGTH = 58656
}
