import { WHITE_PLAYER_ID, BLACK_PLAYER_ID, NUM_OF_PIECES_PER_PLAYER } from './config';

type WizardPosition = number;
type StudentPosition = number;
type CardID = number;
type CardRotated = number;
type CardMoves = number[];

const WIZARD_INDEX: number = 0;
const STUDENTS_OFFSET: number = 1;
// const NUM_OF_WIZARDS: number = 1;
const NUM_OF_STUDENTS: number = 4;
const CARDS_OFFSET: number = 5;
const WHITE_PLAYER_OFFSET: number = 0;
const BLACK_PLAYER_OFFSET: number = 9;
const MIDDLE_CARD_OFFSET: number = 18;

const BOARD_VALID_CELLS: number = 0b11111_11111_11111_11111_11111;

const WHITE_PLAYER_SHRINE_MASK = 0b00000_00000_00000_00000_00100;
const BLACK_PLAYER_SHRINE_MASK = 0b00100_00000_00000_00000_00000;

const VALID_MOVES_FROM_POSITION_MASKS: { [index: number]: number } = {
    [1]: 0b00000_00000_00111_00111_00111,
    [2]: 0b00000_00000_01111_01111_01111,
    [4]: 0b00000_00000_11111_11111_11111,
    [8]: 0b00000_00000_11110_11110_11110,
    [16]: 0b00000_00000_11100_11100_11100,

    [32]: 0b00000_00111_00111_00111_00111,
    [64]: 0b00000_01111_01111_01111_01111,
    [128]: 0b00000_11111_11111_11111_11111,
    [256]: 0b00000_11110_11110_11110_11110,
    [512]: 0b00000_11100_11100_11100_11100,

    [1024]: 0b00111_00111_00111_00111_00111,
    [2048]: 0b01111_01111_01111_01111_01111,
    [4096]: 0b11111_11111_11111_11111_11111,
    [8192]: 0b11110_11110_11110_11110_11110,
    [16384]: 0b11100_11100_11100_11100_11100,

    [32768]: 0b00111_00111_00111_00111_00000,
    [65536]: 0b01111_01111_01111_01111_00000,
    [131072]: 0b11111_11111_11111_11111_00000,
    [262144]: 0b11110_11110_11110_11110_00000,
    [524288]: 0b11100_11100_11100_11100_00000,

    [1048576]: 0b00111_00111_00111_00000_00000,
    [2097152]: 0b01111_01111_01111_00000_00000,
    [4194304]: 0b11111_11111_11111_00000_00000,
    [8388608]: 0b11110_11110_11110_00000_00000,
    [16777216]: 0b11100_11100_11100_00000_00000,
};

const BOARD_COLUMN_MASKS: { [index: string]: number } = {
    A: 0b00001_00001_00001_00001_00001,
    B: 0b00010_00010_00010_00010_00010,
    C: 0b00100_00100_00100_00100_00100,
    D: 0b01000_01000_01000_01000_01000,
    E: 0b10000_10000_10000_10000_10000,
};

const BOARD_ROW_MASKS: { [index: string]: number } = {
    '1': 0b00000_00000_00000_00000_11111,
    '2': 0b00000_00000_00000_11111_00000,
    '3': 0b00000_00000_11111_00000_00000,
    '4': 0b00000_11111_00000_00000_00000,
    '5': 0b11111_00000_00000_00000_00000,
};

export type GameState = [
    WizardPosition,
    StudentPosition,
    StudentPosition,
    StudentPosition,
    StudentPosition,
    CardID,
    CardRotated,
    CardID,
    CardRotated,
    WizardPosition,
    StudentPosition,
    StudentPosition,
    StudentPosition,
    StudentPosition,
    CardID,
    CardRotated,
    CardID,
    CardRotated,
    CardID,
    CardRotated
];

export type CardMovesMap = { [index: CardID]: CardMoves };

const coordinatesToBitWiseShift = (x: number, y: number): number => {
    return x + y * 5;
};

const shiftPosition = (position: number, shiftBy: number): number => {
    if (shiftBy > 0) {
        return position << shiftBy;
    }
    if (shiftBy < 0) {
        return position >> (-1 * shiftBy);
    }
    return position;
};

const getPlayerOffset = (playerID: number): number => {
    return playerID === WHITE_PLAYER_ID ? WHITE_PLAYER_OFFSET : BLACK_PLAYER_OFFSET;
};

const getPlayerCardIndex = (playerID: number, cardIndex: number): number => {
    return getPlayerOffset(playerID) + CARDS_OFFSET + cardIndex * 2;
};

const getPlayerPieceIndex = (playerID: number, pieceIndex: number): number => {
    return getPlayerOffset(playerID) + pieceIndex;
};

const setPlayerPiecePosition = (
    gameState: GameState,
    playerID: number,
    pieceIndex: number,
    newPosition: number
): void => {
    gameState[getPlayerOffset(playerID) + pieceIndex] = newPosition;
};

const setPlayerCard = (
    gameState: GameState,
    playerID: number,
    cardIndex: number,
    cardID: number,
    cardRotation: number
): void => {
    const playerCardIndex = getPlayerCardIndex(playerID, cardIndex);
    gameState[playerCardIndex] = cardID;
    gameState[playerCardIndex + 1] = cardRotation;
};

const setMiddleCard = (gameState: GameState, cardID: number, cardRotation: number): void => {
    gameState[MIDDLE_CARD_OFFSET] = cardID;
    gameState[MIDDLE_CARD_OFFSET + 1] = cardRotation;
};

export const cloneGameState = (gameState: GameState): GameState => {
    return gameState.slice() as GameState;
};

export const getOpponentID = (playerID: number) => {
    return playerID === WHITE_PLAYER_ID ? BLACK_PLAYER_ID : WHITE_PLAYER_ID;
};

export const getPlayerCardID = (gameState: GameState, playerID: number, cardIndex: number): number => {
    return gameState[getPlayerCardIndex(playerID, cardIndex)];
};

export const getPlayerCard = (gameState: GameState, playerID: number, cardIndex: number): number[] => {
    const playerCardIndex = getPlayerCardIndex(playerID, cardIndex);
    return [gameState[playerCardIndex], gameState[playerCardIndex + 1]];
};

export const getMiddleCard = (gameState: GameState): number[] => {
    return [gameState[MIDDLE_CARD_OFFSET], gameState[MIDDLE_CARD_OFFSET + 1]];
};

export const getPlayerPiecePosition = (gameState: GameState, playerID: number, pieceIndex: number): number => {
    return gameState[getPlayerPieceIndex(playerID, pieceIndex)];
};

export const doesPlayerPieceExist = (gameState: GameState, playerID: number, pieceIndex: number): boolean => {
    return gameState[getPlayerPieceIndex(playerID, pieceIndex)] > 0;
};

export const isGameFinished = (gameState: GameState): boolean => {
    const whiteWizardPosition = getPlayerPiecePosition(gameState, WHITE_PLAYER_ID, WIZARD_INDEX);
    const blackWizardPosition = getPlayerPiecePosition(gameState, BLACK_PLAYER_ID, WIZARD_INDEX);
    const isFinished =
        whiteWizardPosition === 0 ||
        (whiteWizardPosition & BLACK_PLAYER_SHRINE_MASK) > 0 ||
        blackWizardPosition === 0 ||
        (blackWizardPosition & WHITE_PLAYER_SHRINE_MASK) > 0;
    return isFinished;
};

export const getPiecePositionOnBoard = (gameState: GameState, playerID: number, pieceIndex: number): string => {
    const piecePosition = getPlayerPiecePosition(gameState, playerID, pieceIndex);
    const column = Object.keys(BOARD_COLUMN_MASKS).find((key) => {
        return (BOARD_COLUMN_MASKS[key] & piecePosition) > 0;
    });
    const row = Object.keys(BOARD_ROW_MASKS).find((key) => {
        return (BOARD_ROW_MASKS[key] & piecePosition) > 0;
    });
    return `${column}${row}`;
};

export const getGameStateScore = (gameState: GameState, maximizingPlayerID: number): number => {
    const score = (() => {
        const whiteWizardPosition = getPlayerPiecePosition(gameState, WHITE_PLAYER_ID, WIZARD_INDEX);
        if (whiteWizardPosition === 0) {
            return -1;
        }
        if ((whiteWizardPosition & BLACK_PLAYER_SHRINE_MASK) > 0) {
            return 1;
        }
        const blackWizardPosition = getPlayerPiecePosition(gameState, BLACK_PLAYER_ID, WIZARD_INDEX);
        if (blackWizardPosition === 0) {
            return 1;
        }
        if ((blackWizardPosition & WHITE_PLAYER_SHRINE_MASK) > 0) {
            return -1;
        }
        const numOfWhitePieces = [...Array(NUM_OF_PIECES_PER_PLAYER)].filter((_, pieceIndex) => {
            const position = getPlayerPiecePosition(gameState, WHITE_PLAYER_ID, pieceIndex);
            return position > 0;
        }).length;
        const numOfBlackPieces = [...Array(NUM_OF_PIECES_PER_PLAYER)].filter((_, pieceIndex) => {
            const position = getPlayerPiecePosition(gameState, BLACK_PLAYER_ID, pieceIndex);
            return position > 0;
        }).length;
        const whitePieceAdvantage = numOfWhitePieces - numOfBlackPieces;
        switch (whitePieceAdvantage) {
            case 4: {
                return 0.8;
            }
            case 3: {
                return 0.6;
            }
            case 2: {
                return 0.4;
            }
            case 1: {
                return 0.2;
            }
            case 0: {
                return 0;
            }
            case -1: {
                return -0.2;
            }
            case -2: {
                return -0.4;
            }
            case -3: {
                return -0.6;
            }
            case -4: {
                return -0.8;
            }
            default: {
            }
        }
        return 0;
    })();
    if (score === 0) {
        return 0;
    }
    return maximizingPlayerID === WHITE_PLAYER_ID ? score : -1 * score;
};

export const isPlayerMoveValid = (
    gameState: GameState,
    cardMovesMap: CardMovesMap,
    playerID: number,
    pieceIndex: number,
    cardIndex: number,
    moveIndex: number
): boolean => {
    const piecePosition = getPlayerPiecePosition(gameState, playerID, pieceIndex);
    const [cardID, cardRotation] = getPlayerCard(gameState, playerID, cardIndex);
    const shiftBy = cardRotation * cardMovesMap[cardID][moveIndex];
    const piecePositionAfterMove =
        shiftPosition(piecePosition, shiftBy) & VALID_MOVES_FROM_POSITION_MASKS[piecePosition];
    if (piecePositionAfterMove === 0) {
        return false;
    }
    for (let otherOwnPieceIndex = 0; otherOwnPieceIndex < NUM_OF_PIECES_PER_PLAYER; otherOwnPieceIndex++) {
        if (pieceIndex === otherOwnPieceIndex) {
            continue;
        }
        const otherOwnPiecePosition = getPlayerPiecePosition(gameState, playerID, otherOwnPieceIndex);
        if (otherOwnPiecePosition === 0) {
            continue;
        }
        if ((piecePositionAfterMove & otherOwnPiecePosition) > 0) {
            return false;
        }
    }
    return true;
};

export const applyPlayerMoveToGameState = (
    gameState: GameState,
    cardMovesMap: CardMovesMap,
    playerID: number,
    pieceIndex: number,
    cardIndex: number,
    moveIndex: number
) => {
    const piecePosition = getPlayerPiecePosition(gameState, playerID, pieceIndex);
    const [cardID, cardRotation] = getPlayerCard(gameState, playerID, cardIndex);
    const shiftBy = cardRotation * cardMovesMap[cardID][moveIndex];
    const piecePositionAfterMove =
        shiftPosition(piecePosition, shiftBy) & VALID_MOVES_FROM_POSITION_MASKS[piecePosition];
    const [middleCardCopyCardID, middleCardCopyIsRotated] = getMiddleCard(gameState);
    setPlayerPiecePosition(gameState, playerID, pieceIndex, piecePositionAfterMove);
    setPlayerCard(gameState, playerID, cardIndex, middleCardCopyCardID, middleCardCopyIsRotated);
    setMiddleCard(gameState, cardID, -1 * cardRotation);
    const opponentID = getOpponentID(playerID);
    for (let opponentPieceIndex = 0; opponentPieceIndex < NUM_OF_PIECES_PER_PLAYER; opponentPieceIndex++) {
        const opponentPiecePosition = getPlayerPiecePosition(gameState, opponentID, opponentPieceIndex);
        if (opponentPiecePosition === 0) {
            continue;
        }
        // moved on opponent piece
        if ((piecePositionAfterMove & opponentPiecePosition) !== 0) {
            setPlayerPiecePosition(gameState, opponentID, opponentPieceIndex, 0);
            return;
        }
    }
};

export const initCardMove = (cardMovesMap: CardMovesMap, cardID: number, x: number, y: number): void => {
    const shiftBy = coordinatesToBitWiseShift(x, y);
    if (shiftBy === 0) {
        return;
    }
    if (!cardMovesMap[cardID]) {
        cardMovesMap[cardID] = [];
    }
    cardMovesMap[cardID].push(shiftBy);
};

export const initPlayerWizard = (gameState: GameState, playerID: number, x: number, y: number): void => {
    setPlayerPiecePosition(gameState, playerID, WIZARD_INDEX, shiftPosition(1, coordinatesToBitWiseShift(x, y)));
};

export const initPlayerStudent = (
    gameState: GameState,
    playerID: number,
    studentIndex: number,
    x: number,
    y: number
): void => {
    setPlayerPiecePosition(
        gameState,
        playerID,
        STUDENTS_OFFSET + studentIndex,
        shiftPosition(1, coordinatesToBitWiseShift(x, y))
    );
};

export const initPlayerCard = (gameState: GameState, playerID: number, cardIndex: number, cardID: number): void => {
    const playerCardIndex = getPlayerCardIndex(playerID, cardIndex);
    gameState[playerCardIndex] = cardID;
    gameState[playerCardIndex + 1] = 1;
};

export const initMiddleCard = (gameState: GameState, cardID: number): void => {
    gameState[MIDDLE_CARD_OFFSET] = cardID;
    gameState[MIDDLE_CARD_OFFSET + 1] = 1;
};
