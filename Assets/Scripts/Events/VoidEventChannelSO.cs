using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

[CreateAssetMenu( menuName = "Events/Void Event Channel" )]
public class VoidEventChannelSO : ScriptableObject
{
    public delegate void Event();

    public Event OnEventRaised;

    public void RaiseEvent()
    {
        OnEventRaised?.Invoke();
    }
}