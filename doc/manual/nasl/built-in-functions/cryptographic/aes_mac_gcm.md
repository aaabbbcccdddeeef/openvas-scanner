# aes_mac_gcm

## NAME

**aes_mac_gcm** - takes two named arguments key, data

## SYNOPSIS

*str* **aes_mac_gcm**(key: str, data: str);

**aes_mac_gcm** It takes four named arguments key, data.

## DESCRIPTION

aes_mac_gcm encrypts given data with given key by using AES in GCM mode.
The according aes algorithm is dependent on the key size (if the key is 128 bit than AES128 is used, 256 and 512 are supported.)


## RETURN VALUE

Encrypted data

## ERRORS

Returns NULL when a given parameter is null.

## SEE ALSO

**[aes_mac_gcm(3)](aes_mac_gcm.md)**
