import binascii
import sys
import curses
def main():
    if len(sys.argv) != 2:
        print("发生错误,需检查参数")
        return

    filename = sys.argv[1]

    with open(filename,'rb') as f:
        content = f.read()
        print("0x"+ str(binascii.hexlify(content),"utf-8"))

if __name__ == "__main__":
    main()
