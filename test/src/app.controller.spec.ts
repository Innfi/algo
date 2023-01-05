// import { Test, TestingModule } from '@nestjs/testing';

// import { AppController } from './app.controller';
// import { AppService } from './app.service';
import { Node, SingleLink } from './link/single_link';
import { DoubleNode, DoubleLink } from './link/double_link';

describe('AppController', () => {
  // let appController: AppController;

  // beforeEach(async () => {
  //   const app: TestingModule = await Test.createTestingModule({
  //     controllers: [AppController],
  //     providers: [AppService],
  //   }).compile();

  //   appController = app.get<AppController>(AppController);
  // });

  describe('single', () => {
    it('create node', () => {
      const data = 'test';
      const node = new Node(data);

      expect(node.data).toBe(data);
    });

    it('link node', () => {
      const data = 'test';
      const root = new Node(data);

      const tailData = 'tail';
      const tail = new Node(tailData);
      root.next = tail;

      expect(root.next.data).toBe(tailData);
    });

    it('initial node: root has same data as tail', () => {
      const instance = new SingleLink();

      const startData = 'start';
      instance.insert(startData);

      expect(instance.root.data).toBe(startData);
      expect(instance.tail.data).toBe(startData);
    });

    it('second node: root and tail have different data', () => {
      const instance = new SingleLink();

      const startData = 'start';
      instance.insert(startData);

      const nextData = 'next';
      instance.insert(nextData);

      expect(instance.root.data).not.toEqual(instance.tail.data);
    });

    it('can access to tail node through root link', () => {
      const instance = new SingleLink();
      instance.insert('first');
      instance.insert('second');

      const tailData = 'third';
      instance.insert(tailData);

      expect(instance.root.next.next.data).toBe(tailData);
    });

    it('delete node in the middle', () => {
      const instance = new SingleLink();
      instance.insert('first');
      instance.insert('second');
      instance.insert('third');

      instance.deleteOne('second');

      expect(instance.root.data).toBe('first');
      expect(instance.root.next.data).toBe('third');
    });

    it('delete first node', () => {
      const instance = new SingleLink();
      instance.insert('first');
      instance.insert('second');
      instance.insert('third');

      instance.deleteOne('first');

      expect(instance.root.data).toBe('second');
      expect(instance.root.next.data).toBe('third');
    });

    it('delete last node', () => {
      const instance = new SingleLink();
      instance.insert('first');
      instance.insert('second');
      instance.insert('third');
      instance.insert('fourth');

      instance.deleteOne('fourth');

      expect(instance.root.next.next.data).toBe('third');
      expect(instance.root.next.next.next).toBeUndefined();
    });
  });

  describe('double', () => {
    it('create node', () => {
      const data = 'initial';
      const node = new DoubleNode(data);

      expect(node.data).toBe(data);
    });

    it('initial node: root has no relation', () => {
      const instance = new DoubleLink();

      const startData = 'start';
      instance.insert(startData);
    });

    it('second node: root and tail is linked', () => {
      const instance = new DoubleLink();

      instance.insert('first');
      instance.insert('second');

      expect(instance.root.data).toBe(instance.root.next.prev.data);
    });

    it('tail node points at the end of the link', () => {
      const instance = new DoubleLink();

      instance.insert('first');
      instance.insert('second');
      expect(instance.tail.data).toBe('second');

      instance.insert('third');

      expect(instance.tail.data).toBe('third');
    });

    it('can access to tail node through root link', () => {
      const instance = new DoubleLink();

      instance.insert('first');
      instance.insert('second');

      const tailData = 'third';
      instance.insert(tailData);

      expect(instance.root.next.next.data).toBe(tailData);
    });

    it('delete root node: next node takes place', () => {
      const instance = new DoubleLink();

      instance.insert('first');
      instance.insert('second');
      instance.insert('third');

      instance.deleteNode('first');
      
      expect(instance.root.data).toBe('second');
      expect(instance.root.prev).toBeUndefined();
    });

    it('delete node in the middle', () => {
      const instance = new DoubleLink();

      instance.insert('first');
      instance.insert('second');
      instance.insert('third');

      instance.deleteNode('second');

      expect(instance.root.data).toBe('first');
      expect(instance.root.next.data).toBe('third');
      expect(instance.root.next.prev.data).toBe('first');
    });

    it('delete node in the tail', () => {
      const instance = new DoubleLink();

      instance.insert('first');
      instance.insert('second');
      instance.insert('third');

      instance.deleteNode('third');

      expect(instance.tail.data).toBe('third');
      expect(instance.tail.prev.data).toBe('second');
    });
  });

});
