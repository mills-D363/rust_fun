#include "stdio.h"
#include "stdlib.h"
#include "string.h"

#define MAXNUMS 16
#define BUFFLEN sizeof(unsigned short) * MAXNUMS

const char RED[] = "\x1b[2;31m";
const char REDB[] = "\x1b[1;31m";
const char GRN[] = "\x1b[0;32m";
const char BLU[] = "\x1b[0;34m";
const char BRN[] = "\x1b[0;33m";
const char GRY[] = "\x1b[2;37m";
const char YEL[] = "\x1b[1;33m";
const char LCY[] = "\x1b[1;36m";
const char NC[] = "\x1b[0m";

unsigned short actRow1 = 0;
unsigned short gSTEPS = 0;

void display_ushort_to_binary(unsigned short n) {
  for (unsigned short i = 0x8000; i > 0; i = i >> 1)
    (n & i) ? printf("  %s1", REDB) : printf("  %s0", NC);
  printf("%s\n", NC);
}

void init_test_matrix(unsigned short *pMatrix) {
  unsigned short i = 0;
  pMatrix[i++] = 0x2802;
  pMatrix[i++] = 0x9803;
  pMatrix[i++] = 0xe60c;
  pMatrix[i++] = 0x1404;
  pMatrix[i++] = 0xd800;
  pMatrix[i++] = 0x5200;
  pMatrix[i++] = 0x0780;
  pMatrix[i++] = 0x0140;
  pMatrix[i++] = 0x0600;
  pMatrix[i++] = 0x0260;
  pMatrix[i++] = 0x0040;
  pMatrix[i++] = 0x0002;
  pMatrix[i++] = 0x2005;
  pMatrix[i++] = 0x3005;
  pMatrix[i++] = 0xc001;
  pMatrix[i++] = 0x4004;
}

void init_empty_matrix(unsigned short *pMatrix) {
  memset(pMatrix, 0, BUFFLEN);
}

void copy_matrix(unsigned short *pMatrix_D, unsigned short *pMatrix_S,
                 size_t nBytes) {
  memcpy(pMatrix_D, pMatrix_S, nBytes);
}

void display_matrix(unsigned short *pMatrix) {
  for (unsigned short i = 0; i < MAXNUMS; i++)
    display_ushort_to_binary(pMatrix[i]);
}

void print_matrix_rows(unsigned short *pMatrix) {
  for (unsigned short i = 0; i < MAXNUMS; i++)
    printf("%s  [%02d] = 0x%04x\n", BLU, i+1, pMatrix[i]);
}

/* iCol: 1 ~ 16 */
void invert_node(unsigned short *pInt, unsigned short iCol) {
  unsigned short iTmp = 0x8000;
  iTmp = iTmp >> --iCol;
  if (*pInt & iTmp)
    *pInt = *pInt & (0xffff ^ iTmp);
  else
    *pInt = *pInt | iTmp;
}

/* iRow: 1 ~ 16, iCol: 1 ~ 16 */
void invert_crossover(unsigned short *pMatrix, unsigned short iRow,
                      unsigned short iCol) {
  iRow--;
  invert_node(pMatrix + iRow, iCol);

  if (iRow - 1 >= 0)
    invert_node(pMatrix + iRow - 1, iCol);
  if (iRow + 1 <= 15)
    invert_node(pMatrix + iRow + 1, iCol);

  if (iCol - 1 >= 1)
    invert_node(pMatrix + iRow, iCol - 1);
  if (iCol + 1 <= 16)
    invert_node(pMatrix + iRow, iCol + 1);

  gSTEPS++;
}

int main() {
  unsigned short iShortInt[MAXNUMS], iShortIntCopy[MAXNUMS];
  unsigned short iRowV, iTraverseVin1stRow, bn, cnt, minSteps;

  char usrChoice[64];

  init_test_matrix(iShortInt);

  //printf("Original:\n");
  display_matrix(iShortInt);

   
      // Backup current latest matrix
      copy_matrix(iShortIntCopy, iShortInt, BUFFLEN);

      minSteps = 65535;
      iTraverseVin1stRow = 1;
      while (1) {
        gSTEPS = 0;
        //printf("[%05d]:\n", iTraverseVin1stRow);

	// Traverse all the possible attempts (1 ~ 65535) for the 1st row first, then process 
	// the following rows one by one till the last row is empty (succeed) or not (fail)
        cnt = 0;
        for (bn = 0x8000; bn > 0; bn = bn >> 1) {
          cnt++;
          if (iTraverseVin1stRow & bn) {
            invert_crossover(iShortInt, 1, cnt);
            //printf("\n  - Updated[1,%d]: ", cnt);
            //display_ushort_to_binary(iShortInt[1]);
          }
	}

        // After the 1st row is processed, process each row one by one for every '1' node by 
        // inverting the crossover centered at the node just beneath it.
        for (iRowV = 1; iRowV < MAXNUMS; iRowV++) {
          cnt = 0;
          for (bn = 0x8000; bn > 0; bn = bn >> 1) {
            cnt++;
            if (iShortInt[iRowV - 1] & bn) {
              invert_crossover(iShortInt, iRowV + 1, cnt);
              //printf("\n  - Updated[%d,%d]: ", iRowV, cnt);
              //display_ushort_to_binary(iShortInt[iRowV]);
            }
          }
        }
        if (iShortInt[MAXNUMS-1] == 0) 
            printf("%s>>> Row1: 0x%04x, Steps: %d <<<\n", BLU, iTraverseVin1stRow, gSTEPS);
        
        // Restore_matrix from backup
        copy_matrix(iShortInt, iShortIntCopy, BUFFLEN);
        // memcpy((unsigned short *)iShortInt, (unsigned short *)iShortIntCopy, BUFFLEN);

        if (gSTEPS < minSteps) {
          actRow1 = iTraverseVin1stRow;
          minSteps = gSTEPS;
        }

	if (iTraverseVin1stRow < 65535)
	  iTraverseVin1stRow++;
	else
	  break;
      }

      printf("\nminSTEPs = %d \n", minSteps);
      printf("Attempt on Row1 (0x%04x): ", actRow1);
      display_ushort_to_binary(actRow1);

  return 0;
}
