using UnityEngine;

[CreateAssetMenu( menuName = "Events/Ints Event Channel" )]
public class IntEventChannelSO : ScriptableObject
{
    public delegate void Event( int[] numbers );

    public event Event OnEventRaised;

    public void RaiseEvent( int[] numbers )
    {
        OnEventRaised?.Invoke( numbers );
    }
}