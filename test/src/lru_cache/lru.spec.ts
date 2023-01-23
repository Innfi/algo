import { LRUCache } from './lru_cache';

describe('lru cache', () => {
	it('adding data like a normal linked list', () => {
		const instance = new LRUCache();

		instance.insert('aaaa');
		instance.insert('bbbb');
		instance.insert('cccc');

		expect(instance.root.data).toBe('cccc');
		expect(instance.root.next.data).toBe('bbbb');
		expect(instance.root.next.next.data).toBe('aaaa');
	});

	it('get data', () => {
		const instance = new LRUCache();

		instance.insert('aaaa');
		instance.insert('bbbb');
		instance.insert('cccc');

		expect(instance.get('bbbb')).toBe('bbbb');
		expect(instance.get('notdata')).toBeUndefined();
	});

	it('referencing data reallocates location to root', () => {
		const instance = new LRUCache();

		const rootData = 'aaaa';
		instance.insert('bbbb');
		instance.insert('cccc');
		instance.insert(rootData);

		expect(instance.root.data).toBe(rootData);

		instance.get('cccc');

		expect(instance.root.data).toBe('cccc');
		expect(instance.root.next.data).toBe('aaaa');
		expect(instance.root.next.next.data).toBe('bbbb');
	});

	it('cache len', () => {
		const instance = new LRUCache();

		instance.insert('aaaa');
		instance.insert('bbbb');
		expect(instance.nodeLen()).toBe(2);

		instance.insert('ccc');
		expect(instance.nodeLen()).toBe(3);
	});

	it('inserting data more than limit deletes tail', () => {
		const instance = new LRUCache(3);

		instance.insert('aaaa');
		instance.insert('bbbb');
		instance.insert('cccc');
		instance.insert('dddd');

		expect(instance.root.next.next.next).toBeUndefined();
	});
});