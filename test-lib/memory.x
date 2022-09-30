MEMORY {
    FLASH : ORIGIN = 0x10000000 + 1024K, LENGTH = 2048K - 1024K
    RAM   : ORIGIN = 0x20000000 + 128K, LENGTH = 256K-128K
}

SECTIONS {
    /* ### Boot loader */
    .boot2 ORIGIN(BOOT2) :
    {
        KEEP(*(.boot2));
    } > BOOT2
} INSERT BEFORE .text;
