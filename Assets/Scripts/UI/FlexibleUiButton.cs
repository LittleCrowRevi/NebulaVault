using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

[RequireComponent(typeof(Button))]
[RequireComponent(typeof(Image))]
public class FlexibleUiButton : FlexibleUi
{
    [Header( "Broadcast Events" )]
    [SerializeField] private VoidEventChannelSO Command;

    public void Start()
    {
        var button = GetComponent< Button >();
        button.onClick.AddListener( () => Command.RaiseEvent() );
    }

    public override void Update()
    {
        
    }
}
