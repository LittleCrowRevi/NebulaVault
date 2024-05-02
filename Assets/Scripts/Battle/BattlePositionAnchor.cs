using UnityEngine;

public enum PositionDirection
{
    Left,
    Right,
    Up,
    Down,
}

public enum AnchorType
{
    Friendly,
    Hostile
}

public class BattlePositionAnchor : MonoBehaviour
{
    public AnchorType        anchorType;
    public PositionDirection direction;
}