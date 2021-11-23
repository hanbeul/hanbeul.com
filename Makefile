OUTNAME = wasm
OUTDIR = dist

all:
	wasm-pack build --target web --out-name ${OUTNAME} --out-dir ./${OUTDIR}
clean:
	rm ./${OUTDIR}/${OUTNAME}*
	rm ./${OUTDIR}/package.json
