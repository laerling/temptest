#include <stdio.h>
#include <string.h>
#include "sha256.h"

static unsigned int const AVG_HITS_PER_VAL = 256;

void hash_str(char* str, BYTE* hash) {
	// init hash
	SHA256_CTX sha256_ctx;
	for(int i=0;i<64;i++)
		sha256_ctx.data[i] = '\0';
	sha256_init(&sha256_ctx);

        // for now we assume str to be shorter than 64
        size_t str_len = strlen(str);

	// update hash
	BYTE data[64];
        strncpy((char*)data, str, str_len);
	sha256_update(&sha256_ctx, (BYTE const*)data, 2);

	// digest hash
	sha256_final(&sha256_ctx, hash);
}

int main() {
        for(size_t bits=1; bits<=64; bits++) {
                unsigned int const iterations = (1 << bits) * AVG_HITS_PER_VAL;
                size_t const bytes_needed = (bits-1) / 8 + 1;
                unsigned char* str = (unsigned char*)"deadbeef";
                for(unsigned int i=0; i<iterations; i++) {

                        // hash
                        BYTE hash[32];
                        // FIXME truncate string to amount of bytes
                        hash_str(str, hash);

                        // print hex hash
                        for(int i=0;i<32;i++)
                                printf("%02x", hash[i]);
                        printf("\n");
                }
        }
}
