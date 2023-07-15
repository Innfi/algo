
export type Fields = string[][];
export interface Position2D {
  x: number;
  y: number;
}

export const VERTICAL_LIMIT = 19;
export const HORIZONTAL_LIMIT = 19;

export interface FieldStatus {
  fields: Fields;
  lastStonePosition?: Position2D;
}

type FlagType = 'O' | 'X';

export enum LineStatus {
  INITIAL = 0,
  TRIPLE,
  TRIPLE_BLOCKED_LEFT,
  TRIPLE_BLOCKED_RIGHT,
  QUAD_BLOCKED_LEFT,
  QUAD_BLOCKED_RIGHT,
}

export interface LineStatusSuggestion {
  status: LineStatus;
  possiblePosition: Position2D;
}

type MoveResultType = 'success' | 'fail';

export interface MoveResult {
  resultType: MoveResultType;
  resultPos: Position2D;
}

export const createNewFields = (ySize: number, xSize: number): string[][] => {
  const result: string[][] = [];
  for (let i = 0; i < ySize; i++) {
    const yArray: Array<"" | "O" | "X"> = [];
    for (let j = 0; j < xSize; j++) {
      yArray.push("");
    }
    result.push(yArray);
  }
  return result;
};

export interface OmPlayer {
  getDescription(): { nickname: string; tactics: string; };

  dropTheStone(
    fieldStatus: FieldStatus,
    yourFlag: "O" | "X",
  ): Promise<Position2D>;
}

export class Runner implements OmPlayer {
  getDescription(): { nickname: string; tactics: string; } {
    return {
      nickname: 'innfi',
      tactics: 'observing'
    };
  }

  async dropTheStone(
    fieldStatus: FieldStatus,
    yourFlag: "O" | "X",
  ): Promise<Position2D> {
    return {
      x: 1,
      y: 1,
    };
  }

  findConditionHorizontal(fieldStatus: FieldStatus, targetFlag: FlagType): Position2D | undefined {
    const fields = fieldStatus.fields;
    const lastPos = fieldStatus.lastStonePosition!;

    let posX = lastPos.x;
    while (posX > 0 && fields[posX][lastPos.y] === targetFlag) {
      posX--;
    }

    if (posX < 0) return undefined;

    return {
      x: posX,
      y: lastPos.y,
    };
  }

  // findLineStatusHorizontal(fieldStatus: FieldStatus, targetFlag: FlagType): LineStatusSuggestion {
  //   const fields = fieldStatus.fields;
  //   const lastPos = fieldStatus.lastStonePosition!;
  // }

}

export const toLeft = (pos: Readonly<Position2D>): MoveResult => {
  if (pos.x <= 0) return { resultType: 'fail', resultPos: pos };

  return {
    resultType: 'success',
    resultPos: { x: pos.x-1, y: pos.y },
  };
};

export const toRight = (pos: Readonly<Position2D>): MoveResult => {
  if (pos.x >= HORIZONTAL_LIMIT) return { resultType: 'fail', resultPos: pos };

  return {
    resultType: 'success',
    resultPos: { x: pos.x+1, y: pos.y },
  };
};

export const toUp = (pos: Readonly<Position2D>): MoveResult => {
  if (pos.y <= 0) return { resultType: 'fail', resultPos: pos };

  return {
    resultType: 'success',
    resultPos: { x: pos.x, y: pos.y-1 },
  };
};

export const toDown = (pos: Readonly<Position2D>): MoveResult => {
  if (pos.y >= VERTICAL_LIMIT) return { resultType: 'fail', resultPos: pos };

  return {
    resultType: 'success',
    resultPos: { x: pos.x, y: pos.y+1 },
  };
};