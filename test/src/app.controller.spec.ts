import { Test, TestingModule } from '@nestjs/testing';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { Node, SingleLink } from '../src/single_link/link';

describe('AppController', () => {
  let appController: AppController;

  beforeEach(async () => {
    const app: TestingModule = await Test.createTestingModule({
      controllers: [AppController],
      providers: [AppService],
    }).compile();

    appController = app.get<AppController>(AppController);
  });

  describe('root', () => {
    it('should return "Hello World!"', () => {
      expect(appController.getHello()).toBe('Hello World!');
    });

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
  });
});
