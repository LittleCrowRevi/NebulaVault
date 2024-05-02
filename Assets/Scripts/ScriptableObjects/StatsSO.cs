using Unity;
using UnityEngine;

[CreateAssetMenu(menuName = "Variables/Stats")]
public class StatsSO : IntVariable
{
    public StatType type;
}

public enum StatType
{
    Focus,
    Mind,
    Body
}