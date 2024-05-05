
using UnityEngine;

public class StringEventChannelSO : ScriptableObject
{
    public delegate void Event(string text);

    public event Event OnEventRaised;

    public void RaiseEvent( string text )
    {
        OnEventRaised?.Invoke( text );
    }
}