BOOT_OBJ=boot.o
BIN=boot.bin

all: boot linking

boot: boot.s
	as -o boot.o boot.s

linking: boot.o
	ld -Tlinker_script.ld -o $(BIN) boot.o
	objcopy --remove-section .note.gnu.property $(BIN)
	objcopy -S -O binary $(BIN) $(BIN)
