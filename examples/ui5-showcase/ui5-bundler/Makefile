dist/build-finished.txt: *.js *.json
	npm install
	npm run build
	# Do this last so we don't get an incomplete build
	touch dist/build-finished.txt
