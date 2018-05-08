
main:
	echo "The Makefile is meant for packaging"

arch_clean:
	rm -f ./packaging/arch/l1gth.tar.gz
	rm -rf  ./packaging/arch/pkg
	rm -rf  ./packaging/arch/src

arch: arch_clean
		tar -czf /tmp/l1gth.tar.gz .
		cp /tmp/l1gth.tar.gz ./packaging/arch/
		cd  ./packaging/arch/  && makepkg -f
		make arch_clean
		echo "Your package is in ./packaging/arch"
