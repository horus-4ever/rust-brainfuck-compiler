if [ ! -d $1/build ]; then
    mkdir $1/build
fi

nasm -f elf temp.asm -o $1/build/brainfuck.o
ld -m elf_i386 -s $1/build/brainfuck.o -o $1/build/brainfuck
rm $1/build/brainfuck.o $1/temp.asm