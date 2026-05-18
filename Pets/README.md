# TermiPet Pets

This directory contains the default pet resource packages for TermiPet.

## Pet Package Format

Each pet is a folder containing:

```
pet-name/
├── pet.json          # Pet configuration
├── spritesheet.webp  # Sprite sheet image (WebP format recommended)
└── preview.png       # Optional preview image
```

## pet.json Format

```json
{
  "id": "unique-pet-id",
  "displayName": "Display Name",
  "description": "Pet description",
  "author": "Author Name",
  "version": "1.0.0",
  "spritesheetPath": "spritesheet.webp",
  "animations": [
    {
      "name": "idle",
      "row": 0,
      "frames": 4,
      "frameDurationMs": 250
    }
  ],
  "defaultAnimation": "idle",
  "interactions": {
    "click": "happy",
    "doubleClick": "celebrating"
  }
}
```

## Sprite Sheet Layout

The sprite sheet should be organized as a grid:
- Each row represents an animation state
- Each column represents a frame in that animation
- Default frame size: 64x64 pixels
- 9 rows for the standard animation states:
  1. Idle
  2. Running
  3. Moving
  4. Happy
  5. Alert
  6. Error
  7. Sleeping
  8. Thinking
  9. Celebrating

## Available Pets

### Terminal Cat
The official mascot of TermiPet. A cute pixel art cat that accompanies you while coding.

## Creating Custom Pets

1. Create a new folder with your pet's ID
2. Create a `pet.json` configuration file
3. Create a sprite sheet following the layout above
4. Import via TermiPet settings

## Petdex Compatibility

TermiPet supports importing pets from the [Petdex](https://petdex.crafter.run/) community.
