import { mkdir, writeFile } from 'node:fs/promises';
const size=32, bitmap=Buffer.alloc(40+size*size*4+size*4);
bitmap.writeUInt32LE(40,0);bitmap.writeInt32LE(size,4);bitmap.writeInt32LE(size*2,8);bitmap.writeUInt16LE(1,12);bitmap.writeUInt16LE(32,14);bitmap.writeUInt32LE(size*size*4,20);
const glyphs=['01110','10001','10000','10000','10000','10001','01110','00000','10001','11011','10101','10101','10001','10001','10001'];
for(let y=0;y<size;y++)for(let x=0;x<size;x++){
  const edge=x<2||x>29||y<2||y>29;
  let color=edge?[20,35,23]:[178,231,133];
  const gx=x-7,gy=y-9;
  if(gy>=0&&gy<14){const row=Math.floor(gy/2);if(gx>=0&&gx<5&&glyphs[row][gx]==='1')color=[20,35,23];if(gx>=8&&gx<13&&glyphs[8+row][gx-8]==='1')color=[20,35,23];}
  const p=40+((size-1-y)*size+x)*4;bitmap[p]=color[2];bitmap[p+1]=color[1];bitmap[p+2]=color[0];bitmap[p+3]=255;
}
const header=Buffer.alloc(22);header.writeUInt16LE(1,2);header.writeUInt16LE(1,4);header[6]=size;header[7]=size;header.writeUInt16LE(1,10);header.writeUInt16LE(32,12);header.writeUInt32LE(bitmap.length,14);header.writeUInt32LE(22,18);
await mkdir('apps/desktop/src-tauri/icons',{recursive:true});await writeFile('apps/desktop/src-tauri/icons/icon.ico',Buffer.concat([header,bitmap]));
