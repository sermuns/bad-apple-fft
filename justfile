extract-frame:
	ffmpeg -y -i bad_apple.webm -vf "select='eq(n,10)'" -vframes 1 -update 1 output.png
