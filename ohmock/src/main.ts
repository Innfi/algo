
export type Fields = string[][];
export interface Position2D {
  x: number;
  y: number;
}

export const VERTICAL_LIMIT = 19;
export const HORIZONTAL_LIMIT = 19;
export const STONE_COUNT_MAX = 5;

export interface FieldStatus {
  fields: Fields;
  lastStonePosition?: Position2D;
}

type FlagType = 'O' | 'X' | '';

export enum LineStatus {
  EMPTY = 0,
  SAFE,
  NEED_BLOCK,
  INVALID_GAME_STATE,
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

  getLineStatus(
    fieldStatus: FieldStatus, 
    targetFlag: FlagType,
    forwardFunc: (pos: Readonly<Position2D>) => MoveResult,
    backwardFunc: (pos: Readonly<Position2D>) => MoveResult,
  ): LineStatusSuggestion {
    const fields = fieldStatus.fields;
    const lastPos = fieldStatus.lastStonePosition!;

    let stoneCount = 1;
    let leftResult: MoveResult = { resultType: 'success', resultPos: lastPos };
    let rightResult: MoveResult = { resultType: 'success', resultPos: lastPos };

    while (true) {
      leftResult = forwardFunc(leftResult.resultPos);
      const { resultType, resultPos } = leftResult;

      if (resultType !== 'success') break;
      if (stoneCount >= STONE_COUNT_MAX) break;
      if (fields[resultPos.x][resultPos.y] !== targetFlag) break;

      stoneCount++;
    }

    while(true) {
      rightResult = backwardFunc(rightResult.resultPos);
      const { resultType, resultPos } = rightResult;

      if (resultType !== 'success') break;
      if (stoneCount >= STONE_COUNT_MAX) break;
      if (fields[resultPos.x][resultPos.y] !== targetFlag) break;

      stoneCount++;
    }

    return this.toLineStatusSuggestion(
      fieldStatus.fields, 
      targetFlag, 
      stoneCount, 
      leftResult, 
      rightResult,
    );
  }

  private toLineStatusSuggestion(
    fields: Fields,
    targetFlag: FlagType,
    stoneCount: number, 
    leftResult: MoveResult, 
    rightResult: MoveResult
  ): LineStatusSuggestion {
    const myFlag: FlagType = targetFlag === 'O' ? 'X' : 'O';

    if (stoneCount >= STONE_COUNT_MAX) {
      return {
        status: LineStatus.INVALID_GAME_STATE,
        possiblePosition: leftResult.resultPos,
      };
    }

    if (fields[leftResult.resultPos.x][leftResult.resultPos.y] !== myFlag) {
      return {
        status: LineStatus.NEED_BLOCK,
        possiblePosition: leftResult.resultPos,
      };
    }

    if (fields[rightResult.resultPos.x][rightResult.resultPos.y] !== myFlag) {
      return {
        status: LineStatus.NEED_BLOCK,
        possiblePosition: rightResult.resultPos,
      };
    }

    return {
      status: LineStatus.SAFE,
      possiblePosition: leftResult.resultPos,
    };
  }
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

export const toDescUp = (pos: Readonly<Position2D>): MoveResult => {
  if (pos.x <= 0) return { resultType: 'fail', resultPos: pos };
  if (pos.y <= 0) return { resultType: 'fail', resultPos: pos };

  return {
    resultType: 'success',
    resultPos: { x: pos.x-1, y: pos.y-1 },
  };
};

export const toDescDown = (pos: Readonly<Position2D>): MoveResult => {
  if (pos.x >= HORIZONTAL_LIMIT) return { resultType: 'fail', resultPos: pos };
  if (pos.y >= VERTICAL_LIMIT) return { resultType: 'fail', resultPos: pos };

  return {
    resultType: 'success',
    resultPos: { x: pos.x+1, y: pos.y+1 },
  };
};

export const toAscUp = (pos: Readonly<Position2D>): MoveResult => {
  if (pos.x >= HORIZONTAL_LIMIT) return { resultType: 'fail', resultPos: pos };
  if (pos.y <= 0) return { resultType: 'fail', resultPos: pos };

  return {
    resultType: 'success',
    resultPos: { x: pos.x+1, y: pos.y-1 },
  };
};

export const toAscDown = (pos: Readonly<Position2D>): MoveResult => {
  if (pos.x <= 0) return { resultType: 'fail', resultPos: pos };
  if (pos.y >= VERTICAL_LIMIT) return { resultType: 'fail', resultPos: pos };

  return {
    resultType: 'success',
    resultPos: { x: pos.x-1, y: pos.y+1 },
  };
};