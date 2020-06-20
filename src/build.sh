if [ ! -d $1/build ]; then
    mkdir $1/build
fi

nasm -f elf32 $1/temp.asm -o $1/build/brainfuck.o
gcc -m32 $1/build/brainfuck.o -o $1/build/brainfuck
rm $1/build/brainfuck.o $1/temp.asm