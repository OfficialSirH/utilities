import type { Schema } from '../schema/Schema';
import { Pointer, type PointerLike } from '../shared/Pointer';
import type { DuplexBuffer } from './DuplexBuffer';

export class SchemaBuffer implements DuplexBuffer {
	#buffer: Uint8Array;
	#bufferView: DataView;
	#schemaBitSizes: number[] = [];
	#byteOffset = 0;
	#bitLength = 0;

	public constructor(maxLength: number) {
		this.#buffer = new Uint8Array(maxLength);
		this.#bufferView = new DataView(this.#buffer.buffer);
	}

	at(index: number): number | undefined {
		return this.#buffer[index];
	}

	public get length(): number {
		return this.#bufferView.buffer.byteLength;
	}

	public get maxBitLength(): number {
		return this.#bufferView.buffer.maxByteLength * 8;
	}

	public get bitLength(): number {
		return this.#bitLength;
	}

	public get schemaBitSizes(): Uint8Array {
		return new Uint8Array(this.#schemaBitSizes);
	}

	public prefixDynamicData(): void {
		this.#schemaBitSizes.push(0);
	}

	public writeBit(value: number): void {
		this.#bufferView.setUint8(this.#byteOffset, value & 1);
		this.#byteOffset += 1;
		this.#schemaBitSizes.push(1);
	}

	public writeInt2(value: number): void {
		this.#bufferView.setUint8(this.#byteOffset, value & 0b11);
		this.#byteOffset += 1;
		this.#schemaBitSizes.push(2);
	}

	public writeInt4(value: number): void {
		this.#bufferView.setUint8(this.#byteOffset, value & 0b1111);
		this.#byteOffset += 1;
		this.#schemaBitSizes.push(4);
	}

	public writeInt8(value: number): void {
		this.#bufferView.setInt8(this.#byteOffset, value & 0xff);
		this.#byteOffset += 1;
		this.#schemaBitSizes.push(8);
	}

	public writeInt16(value: number): void {
		this.#bufferView.setUint16(this.#byteOffset, value & 0xffff);
		this.#byteOffset += 2;
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
	}

	public writeInt32(value: number): void {
		this.#bufferView.setInt32(this.#byteOffset, value);
		this.#byteOffset += 4;
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
	}

	public writeInt64(value: number): void {
		this.#bufferView.setBigInt64(this.#byteOffset, BigInt(value));
		this.#byteOffset += 8;
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
	}

	public writeBigInt32(value: bigint): void {
		this.writeInt32(Number(value));
	}

	public writeBigInt64(value: bigint): void {
		this.#bufferView.setBigInt64(this.#byteOffset, value);
		this.#byteOffset += 8;
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
	}

	public writeFloat32(value: number): void {
		this.#bufferView.setFloat32(this.#byteOffset, value);
		this.#byteOffset += 4;
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
	}

	public writeFloat64(value: number): void {
		this.#bufferView.setFloat64(this.#byteOffset, value);
		this.#byteOffset += 8;
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
		this.#schemaBitSizes.push(8);
	}

	public readBit(offset: PointerLike): 0 | 1 {
		const ptr = Pointer.from(offset);
		const value = (this.#bufferView.getUint8(ptr.value) & 1) as 0 | 1;
		ptr.add(1);
		return value;
	}

	public readInt2(offset: PointerLike): number {
		return this.readUint2(offset);
	}

	public readUint2(offset: PointerLike): number {
		const ptr = Pointer.from(offset);
		const value = this.#bufferView.getUint8(ptr.value) & 0b11; // Read the first 2 bits
		ptr.add(1);
		return value;
	}

	public readInt4(offset: PointerLike): number {
		return this.readUint4(offset);
	}

	public readUint4(offset: PointerLike): number {
		const ptr = Pointer.from(offset);
		const value = this.#bufferView.getUint8(ptr.value) & 0b1111; // Read the first 4 bits
		ptr.add(1);
		return value;
	}

	public readInt8(offset: PointerLike): number {
		return this.readUint8(offset);
	}

	public readUint8(offset: PointerLike): number {
		const ptr = Pointer.from(offset);
		const value = this.#bufferView.getInt8(ptr.value) & 0xff;
		ptr.add(1);
		return value;
	}

	public readInt16(offset: PointerLike): number {
		return this.readUint16(offset);
	}

	public readUint16(offset: PointerLike): number {
		const ptr = Pointer.from(offset);
		const value = this.#bufferView.getUint16(ptr.value);
		ptr.add(2);
		return value;
	}

	public readInt32(offset: PointerLike): number {
		const ptr = Pointer.from(offset);
		const value = this.#bufferView.getInt32(ptr.value);
		ptr.add(4);
		return value;
	}

	public readUint32(offset: PointerLike): number {
		const ptr = Pointer.from(offset);
		const value = this.#bufferView.getUint32(ptr.value);
		ptr.add(4);
		return value;
	}

	public readInt64(offset: PointerLike): number {
		return Number(this.readBigInt64(offset));
	}

	public readUint64(offset: PointerLike) {
		return Number(this.readBigUint64(offset));
	}

	public readBigInt32(offset: PointerLike): bigint {
		const ptr = Pointer.from(offset);
		const value = this.#bufferView.getBigInt64(ptr.value) & BigInt(0xffffffff); // Read the first 4 bytes as a signed 32-bit integer
		ptr.add(4);
		return value;
	}

	public readBigUint32(offset: PointerLike): bigint {
		const ptr = Pointer.from(offset);
		const value = this.#bufferView.getBigUint64(ptr.value) & BigInt(0xffffffff); // Read the first 4 bytes as an unsigned 32-bit integer
		ptr.add(4);
		return value;
	}

	public readBigInt64(offset: PointerLike): bigint {
		const ptr = Pointer.from(offset);
		const value = this.#bufferView.getBigInt64(ptr.value);
		ptr.add(8);
		return value;
	}

	public readBigUint64(offset: PointerLike): bigint {
		const ptr = Pointer.from(offset);
		const value = this.#bufferView.getBigUint64(ptr.value);
		ptr.add(8);
		return value;
	}

	public readFloat32(offset: PointerLike): number {
		const ptr = Pointer.from(offset);
		const value = this.#bufferView.getFloat32(ptr.value);
		ptr.add(4);
		return value;
	}

	public readFloat64(offset: PointerLike): number {
		const ptr = Pointer.from(offset);
		const value = this.#bufferView.getFloat64(ptr.value);
		ptr.add(8);
		return value;
	}

	public toString() {
		let result = '';
		const bufferArray = new Uint8Array(this.#bufferView.buffer, 0, this.length);
		for (let i = 0; i < this.length; i++) {
			result += String.fromCharCode(bufferArray[i]);
		}

		return result;
	}

	public toArray(): Uint8Array {
		return new Uint8Array(this.#bufferView.buffer, 0, this.length);
	}

	public static from(schema: Schema, value: string | DuplexBuffer): DuplexBuffer {
		if (typeof value !== 'string') return value;

		const buffer = new SchemaBuffer(value.length);
		for (let i = 0; i < value.length; i++) {
			buffer.writeInt16(value.charCodeAt(i));
		}

		buffer.#bitLength = value.length << 3;
		return buffer;
	}
}
