#include <stdio.h>
#include <string.h>
#include "sha256.h"

int main() {
	// init hash
	SHA256_CTX sha256_ctx;
	for(int i=0;i<64;i++)
		sha256_ctx.data[i] = '\0';
	sha256_init(&sha256_ctx);

	// update hash
	BYTE data[64];
	data[0] = 'h';
	data[1] = 'i';
	sha256_update(&sha256_ctx, (const BYTE*)data, 2);

	// digest hash
	BYTE hash[32];
	sha256_final(&sha256_ctx, hash);

	// print hex hash
	for(int i=0;i<32;i++)
		printf("%02x", hash[i]);
	printf("\n");
}
